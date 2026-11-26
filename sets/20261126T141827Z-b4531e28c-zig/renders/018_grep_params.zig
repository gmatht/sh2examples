const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

var sh2_io: ?std.Io = null;

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

fn sh2Redirect() []const u8 {
    sh2TODO("redirect");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var count: []const u8 = "";
    var found: []const u8 = "";
    var lineno: []const u8 = "";
    var longline: []const u8 = "";
    var matched: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    try stdout.print("{s}\n", .{"== Basic grep parameters =="});
    try stdout.flush();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = ((sh2Truthy(sh2Pipeline()) and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
    count = sh2Capture();
    try stdout.print("{s}{s}{s}\n", .{"  -v count: ", count, " (expected 2)"});
    try stdout.flush(); // line 10
    matched = sh2Capture();
    try stdout.print("{s}{s}{s}\n", .{"  -c count: ", matched, " (expected 2)"});
    try stdout.flush(); // line 13
    try stdout.print("{s}\n", .{"== Context parameters =="});
    try stdout.flush();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}{s}{s}\n", .{"  -A 2 lines: ", sh2Capture(), " (expected 3)"});
    try stdout.flush(); // line 17
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}{s}{s}\n", .{"  -B 2 lines: ", sh2Capture(), " (expected 3)"});
    try stdout.flush(); // line 20
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}{s}{s}\n", .{"  -C 1 lines: ", sh2Capture(), " (expected 3)"});
    try stdout.flush(); // line 23
    try stdout.print("{s}\n", .{"== File handling parameters =="});
    try stdout.flush();
    try std.fs.cwd().writeFile(.{ .sub_path = "/tmp/grep_file.txt", .data = "content\n" });
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = ((sh2Truthy(sh2Exec()) and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = ((sh2Truthy(sh2Exec()) and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = ((sh2Truthy(sh2Exec()) and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
    try stdout.print("{s}\n", .{"== Output formatting parameters =="});
    try stdout.flush();
    matched = sh2Capture();
    try stdout.print("{s}{s}{s}\n", .{"  -o match: '", matched, "' (expected 'pattern')"});
    try stdout.flush(); // line 33
    lineno = sh2Capture();
    try stdout.print("{s}{s}{s}\n", .{"  -n line: ", lineno, " (expected 1)"});
    try stdout.flush(); // line 36
    try stdout.print("{s}\n", .{"== Recursive parameters =="});
    try stdout.flush();
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): redirect → sh2.redirect
    _ = (sh2Truthy(sh2Exec()) and sh2Truthy(sh2Redirect()));
    found = sh2Capture();
    try stdout.print("{s}{s}{s}\n", .{"  -r recursive: ", found, " files matched (expected 1)"});
    try stdout.flush(); // line 41
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", "/tmp/grep_sub"}); // line 42
    try stdout.print("{s}\n", .{"== Line length parameters =="});
    try stdout.flush();
    longline = sh2Capture();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = ((sh2Truthy(sh2Pipeline()) and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
    try stdout.print("{s}\n", .{"== Word-regexp and line-regexp parameters =="});
    try stdout.flush();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}{s}{s}\n", .{"  -w word match lines: ", sh2Capture(), " (expected 1)"});
    try stdout.flush(); // line 50
    _ = sh2Run(&[_][]const u8{"rm", "-f", "/tmp/grep_file.txt", "/tmp/grep_out.txt"}); // line 52
    _ = &longline;
}
// 42 construct(s) lowered to TODO markers