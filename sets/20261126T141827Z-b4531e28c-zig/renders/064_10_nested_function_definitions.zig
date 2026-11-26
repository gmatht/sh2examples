const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
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

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
    return "";
}


fn inner_func(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    _ = __args;
    var result: i64 = 0;
    _ = sh2Run(&[_][]const u8{"local", "inner_var=", "inner"});
    try stdout.print("{s}{s}{s}{s}\n", .{"Outer: ", sh2Env("outer_var"), ", Inner: ", inner_var});
    try stdout.flush();
    result = (sh2ToInt(sh2GetVar()) + sh2ToInt(inner_var));
    try stdout.print("{s}{d}\n", .{"Result: ", result});
    try stdout.flush();
}
fn outer_func(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    _ = sh2Run(&[_][]const u8{"local", "outer_var=", "outer"});
    _ = sh2Run(&[_][]const u8{"inner_func"});
}
pub fn main(init: std.process.Init) !void {

    const inner_var: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    _ = sh2Run(&[_][]const u8{"outer_func"}); // line 20
}