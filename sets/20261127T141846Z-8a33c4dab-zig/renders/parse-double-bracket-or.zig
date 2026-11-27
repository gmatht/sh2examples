const std = @import("std");

fn sh2FileTest(flag: []const u8, p: []const u8) bool {
    const st = std.fs.cwd().statFile(p) catch return false;
    if (std.mem.eql(u8, flag, "-f")) return st.kind == .file;
    if (std.mem.eql(u8, flag, "-d")) return st.kind == .directory;
    if (std.mem.eql(u8, flag, "-e")) return true;
    if (std.mem.eql(u8, flag, "-s")) return st.size != 0;
    return false;
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    if (((sh2FileTest("-d", "/path1") or sh2FileTest("-d", "/path2")) or sh2FileTest("-d", "/path3"))) {
        try stdout.print("{s}\n", .{"found"});
    try stdout.flush();
    }
    try stdout.print("{s}{s}\n", .{"status: ", "ok"});
    try stdout.flush();
}