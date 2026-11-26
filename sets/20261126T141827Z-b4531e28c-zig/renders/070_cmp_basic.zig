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

pub fn main(init: std.process.Init) !void {

    var __ps_tmp0: []const u8 = "";
    var __ps_tmp1: []const u8 = "";
    var __ps_tmp2: []const u8 = "";
    var __ps_tmp3: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try std.fs.cwd().writeFile(.{ .sub_path = "/tmp/cmp_a.txt", .data = "abcdefghij\n" });
    try std.fs.cwd().writeFile(.{ .sub_path = "/tmp/cmp_same.txt", .data = "abcdefghij\n" });
    try std.fs.cwd().writeFile(.{ .sub_path = "/tmp/cmp_diff.txt", .data = "abcdeZghij\n" });
    try std.fs.cwd().writeFile(.{ .sub_path = "/tmp/cmp_diff2.txt", .data = "xyzdefghij\n" });
    try std.fs.cwd().writeFile(.{ .sub_path = "/tmp/cmp_short.txt", .data = "abc\n" });
    _ = sh2Run(&[_][]const u8{"touch", "/tmp/cmp_empty.txt"}); // line 11
    _ = sh2Run(&[_][]const u8{"cmp", "/tmp/cmp_a.txt", "/tmp/cmp_same.txt"}); // line 16
    try stdout.print("{s}{s}\n", .{"exit: ", sh2Env("?")});
    try stdout.flush(); // line 17
    _ = sh2Run(&[_][]const u8{"cmp", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt"}); // line 20
    try stdout.print("{s}{s}\n", .{"exit: ", sh2Env("?")});
    try stdout.flush(); // line 21
    _ = sh2Run(&[_][]const u8{"cmp", "/tmp/cmp_a.txt", "/tmp/cmp_empty.txt"}); // line 24
    try stdout.print("{s}{s}\n", .{"exit: ", sh2Env("?")});
    try stdout.flush(); // line 25
    _ = sh2Run(&[_][]const u8{"cmp", "/tmp/cmp_empty.txt", "/tmp/cmp_empty.txt"}); // line 28
    try stdout.print("{s}{s}\n", .{"exit: ", sh2Env("?")});
    try stdout.flush(); // line 29
    _ = sh2Run(&[_][]const u8{"cmp", "-s", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt"}); // line 32
    try stdout.print("{s}{s}\n", .{"-s exit: ", sh2Env("?")});
    try stdout.flush(); // line 33
    _ = sh2Run(&[_][]const u8{"cmp", "-s", "/tmp/cmp_a.txt", "/tmp/cmp_same.txt"}); // line 34
    try stdout.print("{s}{s}\n", .{"-s same exit: ", sh2Env("?")});
    try stdout.flush(); // line 35
    _ = sh2Run(&[_][]const u8{"cmp", "-l", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt"}); // line 38
    try stdout.print("{s}{s}\n", .{"-l exit: ", sh2Env("?")});
    try stdout.flush(); // line 39
    _ = sh2Run(&[_][]const u8{"cmp", "-b", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt"}); // line 42
    try stdout.print("{s}{s}\n", .{"-b exit: ", sh2Env("?")});
    try stdout.flush(); // line 43
    _ = sh2Run(&[_][]const u8{"cmp", "-n", "5", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt"}); // line 47
    try stdout.print("{s}{s}\n", .{"-n 5 exit: ", sh2Env("?")});
    try stdout.flush(); // line 48
    _ = sh2Run(&[_][]const u8{"cmp", "-n", "10", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt"}); // line 50
    try stdout.print("{s}{s}\n", .{"-n 10 exit: ", sh2Env("?")});
    try stdout.flush(); // line 51
    _ = sh2Run(&[_][]const u8{"cmp", "-n", "10", "/tmp/cmp_a.txt", "/tmp/cmp_short.txt"}); // line 53
    try stdout.print("{s}{s}\n", .{"-n 10 short exit: ", sh2Env("?")});
    try stdout.flush(); // line 54
    _ = sh2Run(&[_][]const u8{"cmp", "-i", "6", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt"}); // line 58
    try stdout.print("{s}{s}\n", .{"-i 6 exit: ", sh2Env("?")});
    try stdout.flush(); // line 59
    _ = sh2Run(&[_][]const u8{"cmp", "-i", "3", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt"}); // line 61
    try stdout.print("{s}{s}\n", .{"-i 3 exit: ", sh2Env("?")});
    try stdout.flush(); // line 62
    _ = sh2Run(&[_][]const u8{"cmp", "-i", "0:6", "/tmp/cmp_a.txt", "/tmp/cmp_diff.txt"}); // line 66
    try stdout.print("{s}{s}\n", .{"-i 0:6 exit: ", sh2Env("?")});
    try stdout.flush(); // line 67
    _ = sh2Run(&[_][]const u8{"cmp", "-i", "5:0", "/tmp/cmp_a.txt", "/tmp/cmp_diff2.txt"}); // line 69
    try stdout.print("{s}{s}\n", .{"-i 5:0 exit: ", sh2Env("?")});
    try stdout.flush(); // line 70
    __ps_tmp0 = sh2Capture();
    try std.fs.cwd().writeFile(.{ .sub_path = __ps_tmp0, .data = "a\n" });
    __ps_tmp1 = sh2Capture();
    try std.fs.cwd().writeFile(.{ .sub_path = __ps_tmp1, .data = "a\n" });
    _ = sh2Run(&[_][]const u8{"cmp", "-s", __ps_tmp0, __ps_tmp1});
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    try stdout.print("{s}{s}\n", .{"aa -s exit: ", sh2Env("?")});
    try stdout.flush(); // line 74
    __ps_tmp2 = sh2Capture();
    try std.fs.cwd().writeFile(.{ .sub_path = __ps_tmp2, .data = "b\n" });
    __ps_tmp3 = sh2Capture();
    try std.fs.cwd().writeFile(.{ .sub_path = __ps_tmp3, .data = "c\n" });
    _ = sh2Run(&[_][]const u8{"cmp", "-s", __ps_tmp2, __ps_tmp3});
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    try stdout.print("{s}{s}\n", .{"bc -s exit: ", sh2Env("?")});
    try stdout.flush(); // line 76
    _ = sh2Run(&[_][]const u8{"rm", "-f", "/tmp/cmp_a.txt", "/tmp/cmp_same.txt", "/tmp/cmp_diff.txt", "/tmp/cmp_diff2.txt", "/tmp/cmp_short.txt", "/tmp/cmp_empty.txt"}); // line 79
}
// 4 construct(s) lowered to TODO markers