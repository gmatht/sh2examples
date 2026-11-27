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

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var __ps_tmp0: []const u8 = "";
    var matched: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{"foo123 bar456"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-E", "(foo|bar)[0-9]+"});
    try stdout.print("{s}\n", .{"a+b*c?"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-F", "a+b*c?"});
    try stdout.print("{s}\n", .{"word wordly subword"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-w", "word"});
    try stdout.print("{s}{s}{s}\n", .{"-e", " ", "exact whole line\\npartial line"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-x", "exact whole line"});
    try stdout.print("{s}{s}{s}\n", .{"-e", " ", "error message\\nwarning message\\ninfo message"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-E", "error|warning"});
    __ps_tmp0 = sh2Capture();
    try std.fs.cwd().writeFile(.{ .sub_path = __ps_tmp0, .data = "error\nwarning\n" });
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    try stdout.print("{s}\n", .{"file123.txt backup456.bak"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-E", "([a-z]+)([0-9]+)\\.([a-z]+)"});
    matched = sh2Capture();
    try stdout.print("{s}{s}\n", .{"  grep_exit: ", sh2Env("?")});
    try stdout.flush(); // line 28
    try stdout.print("{s}{s}\n", .{"  match_count: ", matched});
    try stdout.flush(); // line 29
}
// 4 construct(s) lowered to TODO markers