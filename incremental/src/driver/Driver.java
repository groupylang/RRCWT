package driver;

import front_end.ParserResult;
import front_end.RecursiveDescentParser;
import front_end.ParsingException;
import io.Reader;
import io.Writer;
import middle_end.AssemblyBuilder;
import middle_end.Function;
import middle_end.Module;

import java.io.BufferedOutputStream;
import java.io.DataOutputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class Driver {
    static void help() {
        System.out.println("select a file to compile or 1 option from the ones below:");
        System.out.println("  \"-h\": show help");
        System.out.println("  \"-v\": show version");
        System.out.println("for extra logs about compilation in order to debug, add \"-d\" option to the command.");
    }

    static void version() {
        System.out.println("incremental");
        System.out.println("  a toy language processor");
        System.out.println();
        System.out.println("  version = \"0.1.0\"");
        System.out.println("  authors = [\"sKyrBBit <iamskyrabbit@gmail.com>\"]");
        System.out.println("  license = \"MIT\"");
    }

    static ParserResult grp2ast(String file_name) throws IOException {
        final String input = Reader.use(file_name + ".grp", Reader::read);
        final RecursiveDescentParser parser = new RecursiveDescentParser();
        ParserResult result = null;
        try {
            result = parser.parse(input);
        } catch (ParsingException exception) {
            System.out.println(exception.toString()); // TODO show line, position
            System.exit(1);
        }
        return result;
    }

    static void put_ast(String file_name, List<ast.FunctionDeclare> ast_result) throws IOException {
        final StringBuilder builder = new StringBuilder();
        ast_result.stream()
                .map(ast.FunctionDeclare::toS)
                .map(s -> s + "\n")
                .forEach(builder::append);
        Writer.use(file_name + ".ast", writer -> writer.write(builder.toString()));
    }

    static List<ir.Function> ast2ir(List<ast.FunctionDeclare> ast_result) {
        return ast_result.stream()
                .map(ast.FunctionDeclare::toIR)
                .collect(Collectors.toList());
    }
    static middle_end.Module ast2ir_mid(List<ast.FunctionDeclare> ast_result) {
        return new middle_end.Module(ast_result.stream()
                .map(ast.FunctionDeclare::gen)
                .collect(Collectors.toList()));
    }
    static void put_ir(String file_name, List<ir.Function> ir_result, String[] strings) throws IOException {
        StringBuilder builder = new StringBuilder();
        for (int i = 0; i < strings.length; i++) {
            builder.append('s')
                    .append(i)
                    .append(" = \"")
                    .append(strings[i])
                    .append("\"\n");
        }
        ir_result.stream()
                .map(ir.Function::toString)
                .forEach(builder::append);
        Writer.use(file_name + ".ir", writer -> writer.write(builder.toString()));
    }
    static void ir2wc_and_put(String file_name, List<ir.Function> ir_result, String[] strings) throws IOException {
        final DataOutputStream dos = new DataOutputStream(new BufferedOutputStream(new FileOutputStream(file_name + ".wc")));
        dos.writeInt(0x52435754);
        dos.writeShort(4 * ir_result.stream().mapToInt(ir.Function::instructions_size).sum());
        dos.writeShort(Arrays.stream(strings).mapToInt(s -> s.length() + 1).sum());
        dos.writeShort(ir_result.size() + strings.length);
        dos.writeShort(0x0000);
        dos.writeShort(0x0000);
        for (final ir.Function fn: ir_result) {
            dos.writeBytes(fn.name + "\0");
            dos.writeShort(0x0000);
            dos.writeShort(0x0000); // TODO
        }
        for (int i = 0; i < strings.length; i++) {
            dos.writeBytes("s" + i + "\0");
            dos.writeShort(0x0001);
            dos.writeShort(0x0000); // TODO
        }
        for (final ir.Function fn: ir_result) {
            for (int instr : fn.toWC()) {
                dos.writeInt(instr);
            }
        }
        for (final String str : strings) {
            dos.writeBytes(str + "\0");
        }
        dos.flush();
    }
    static void ir2assembly_and_put(String file_name, List<ir.Function> ir_result, String[] strings) throws IOException {
        StringBuilder assembly_result = new StringBuilder(".intel_syntax noprefix\n");
        for (int i = 0; i < strings.length; i++) {
            assembly_result.append(".Lc")
                    .append(i)
                    .append(":\n")
                    .append("\t.ascii \"")
                    .append(strings[i])
                    .append("\\0\"\n");
        }
        ir_result.stream()
                .map(ir.Function::toAssembly)
                .forEach(assembly_result::append);
        Writer.use(file_name + ".s", writer -> writer.write(assembly_result.toString()));
    }
    public static void main(String[] args) throws IOException {
        if (args.length == 1) {
            if (args[0].equals("-h")) help();
            else if (args[0].equals("-v")) version();
            else {
                ParserResult ast_result = grp2ast(args[0]);
                List<ir.Function> ir_result = ast2ir(ast_result.ast);
//                middle_end.Module ir_mid_result = ast2ir_mid(ast_result.ast);
//                ir_mid_result.toAssembly();
//                System.out.println(AssemblyBuilder.build());
                ir2wc_and_put(args[0], ir_result, ast_result.strings);
                ir2assembly_and_put(args[0], ir_result, ast_result.strings);
            }
        } else if (args.length == 2) {
            if (args[0].equals("-d")) {
                ParserResult ast_result = grp2ast(args[1]);
                put_ast(args[1], ast_result.ast);
                List<ir.Function> ir_result = ast2ir(ast_result.ast);
                put_ir(args[1], ir_result, ast_result.strings);
                ir2wc_and_put(args[1], ir_result, ast_result.strings);
                ir2assembly_and_put(args[1], ir_result, ast_result.strings);
            }
            else if (args[1].equals("-d")) {
                ParserResult ast_result = grp2ast(args[0]);
                put_ast(args[0], ast_result.ast);
                List<ir.Function> ir_result = ast2ir(ast_result.ast);
                put_ir(args[0], ir_result, ast_result.strings);
                ir2wc_and_put(args[0], ir_result, ast_result.strings);
                ir2assembly_and_put(args[0], ir_result, ast_result.strings);
            }
            else help();
        } else {
            help();
        }
    }
}
