const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

fn sh2ArithStub() i64 {
    sh2TODO("arith");
    return 0;
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2Join() []const u8 {
    sh2TODO("join");
    return "";
}

fn sh2ListVar() []const u8 {
    sh2TODO("listVar");
    return "";
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

fn sh2StrHasPrefix() []const u8 {
    sh2TODO("strHasPrefix");
    return "";
}


fn complex_function(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    var i: []const u8 = "";
    // TODO(unsupported): listVar → sh2.listVar
    // TODO(unsupported): listVar → sh2.listVar
    _ = sh2Run(&[_][]const u8{"local", "-a", [_] []const u8{sh2ListVar()}});
    _ = sh2Run(&[_][]const u8{"local", "-A", [_] []const u8{}});
    i = "0";
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    while (sh2Truthy(sh2Exec())) {
        // TODO(unsupported): param → sh2.param
        // TODO(unsupported): param → sh2.param
        __sh2_casesubj_0 = std.fmt.allocPrint(std.heap.page_allocator, "{s}", .{sh2Param()}) catch "";
        // TODO(unsupported): strHasPrefix → sh2.strHasPrefix
        // TODO(unsupported): strHasPrefix → sh2.strHasPrefix
        if (sh2Truthy(sh2StrHasPrefix())) {
            // TODO(unsupported): param → sh2.param
            // TODO(unsupported): param → sh2.param
            _ = sh2Run(&[_][]const u8{"local", "key=", sh2Param()});
            // TODO(unsupported): param → sh2.param
            // TODO(unsupported): param → sh2.param
            _ = sh2Run(&[_][]const u8{"local", "value=", sh2Param()});
            options___key__ = sh2Env("value");
            i = sh2IntStr(sh2ArithStub());
        // TODO(unsupported): strHasPrefix → sh2.strHasPrefix
        // TODO(unsupported): strHasPrefix → sh2.strHasPrefix
        } else if (sh2Truthy(sh2StrHasPrefix())) {
            // TODO(unsupported): param → sh2.param
            // TODO(unsupported): param → sh2.param
            _ = sh2Run(&[_][]const u8{"local", "flags=", sh2Param()});
            j = "0";
            // TODO(unsupported): exec → sh2.exec
            // TODO(unsupported): exec → sh2.exec
            while (sh2Truthy(sh2Exec())) {
                options____flags_j_1___ = "true";
                j = sh2IntStr(sh2ArithStub());
            }
            i = sh2IntStr(sh2ArithStub());
        } else {
            break;
        }
    }
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    return std.fmt.allocPrint(std.heap.page_allocator, "Processed {s} options", .{sh2Join()}) catch "";
}
pub fn main(init: std.process.Init) !void {

    const __sh2_casesubj_0: []const u8 = "";
    const j: []const u8 = "";
    const options___key__: []const u8 = "";
    const options____flags_j_1___: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{complex_function(init, &[_][]const u8{ "--flag1", "--option1=value1", "-abc" })});
    try stdout.flush();
}
// 20 construct(s) lowered to TODO markers