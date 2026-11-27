const std = @import("std");

var sh2_io: ?std.Io = null;

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"bash", "examples/005_args.sh", "one"}); // line 1
    _ = sh2Run(&[_][]const u8{"bash", "examples/005_args.sh", "one", "two"}); // line 2
    _ = sh2Run(&[_][]const u8{"bash", "examples/005_args.sh", "one", "two", "three"}); // line 3
    _ = sh2Run(&[_][]const u8{"bash", "examples/005_args.sh", "1"}); // line 4
    _ = sh2Run(&[_][]const u8{"bash", "examples/005_args.sh", "1", "2", "3"}); // line 5
    _ = sh2Run(&[_][]const u8{"bash", "examples/005_args.sh", "1", "two", "3"}); // line 6
    _ = sh2Run(&[_][]const u8{"bash", "examples/005_args.sh", "A 'quoted' Sting"});
    _ = sh2Run(&[_][]const u8{"bash", "examples/005_args.sh", "A 'quoted' Sting", "2", "3", "4", "5", "6"});
    try stdout.print("{s}{s}\n", .{"status: ", "ok"});
    try stdout.flush();
}