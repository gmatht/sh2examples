const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
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

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"eval", "\n    doselect() {\n        select select_result; do\n            case $select_result in\n                \"\") echo >&2 \"Please enter a number.\" ;;\n                ?*) break ;;\n            esac\n        done\n    }\n"}); // line 5
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}", .{"doselect definition parsed OK\n"});
    try stdout.flush(); // line 17
}
// 2 construct(s) lowered to TODO markers