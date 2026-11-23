const std = @import("std");

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

fn sh2FileTest(flag: []const u8, p: []const u8) bool {
    const st = std.fs.cwd().statFile(p) catch return false;
    if (std.mem.eql(u8, flag, "-f")) return st.kind == .file;
    if (std.mem.eql(u8, flag, "-d")) return st.kind == .directory;
    if (std.mem.eql(u8, flag, "-e")) return true;
    if (std.mem.eql(u8, flag, "-s")) return st.size != 0;
    return false;
}

pub fn main(init: std.process.Init) !void {

    var i: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{s}\n", .{"Hello, World!"});
    try stdout.flush();
    if (sh2FileTest("-f", "test.txt")) {
        try stdout.print("{s}\n", .{"File exists"});
    try stdout.flush();
    }
    {
        const sh2_items0 = [_]i64{1, 2, 3, 4, 5};
        for (sh2_items0[0..]) |sh2_v1| {
            i = sh2_v1;
            try stdout.print("{s}\n", .{sh2IntStr(i)});
    try stdout.flush();
        }
    }
}