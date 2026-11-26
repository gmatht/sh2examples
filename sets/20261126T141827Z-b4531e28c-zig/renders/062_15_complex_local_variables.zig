const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
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

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}


fn test_locals(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    _ = sh2Run(&[_][]const u8{"local", "var1=", (if (__args.len >= 1) __args[1 - 1] else "")});
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    _ = sh2Run(&[_][]const u8{"local", "var2=", sh2Param()});
    _ = sh2Run(&[_][]const u8{"local", "var3=", sh2Capture()});
    try stdout.print("{s}{s}\n", .{"Var1: ", sh2Env("var1")});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"Var2: ", sh2Env("var2")});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"Var3: ", sh2Env("var3")});
    try stdout.flush();
}
pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    _ = sh2Run(&[_][]const u8{"test_locals", "hello", "world"});
}
// 2 construct(s) lowered to TODO markers