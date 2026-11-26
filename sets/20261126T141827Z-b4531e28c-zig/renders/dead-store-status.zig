const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

var sh2_io: ?std.Io = null;

fn sh2FileTest(flag: []const u8, p: []const u8) bool {
    const st = std.fs.cwd().statFile(p) catch return false;
    if (std.mem.eql(u8, flag, "-f")) return st.kind == .file;
    if (std.mem.eql(u8, flag, "-d")) return st.kind == .directory;
    if (std.mem.eql(u8, flag, "-e")) return true;
    if (std.mem.eql(u8, flag, "-s")) return st.size != 0;
    return false;
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

pub fn main(init: std.process.Init) !void {

    var dead: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"rm", "-f", "/tmp/dead-store-elim-probe"}); // line 4
    dead = sh2Capture(); // line 5
    if (sh2FileTest("-e", "/tmp/dead-store-elim-probe")) { // line 6
        try stdout.print("{s}\n", .{"side-effect"});
    try stdout.flush();
    }
    _ = sh2Run(&[_][]const u8{"rm", "-f", "/tmp/dead-store-elim-probe"}); // line 9
    _ = &dead;
}