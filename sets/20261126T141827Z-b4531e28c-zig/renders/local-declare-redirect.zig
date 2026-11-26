const std = @import("std");

fn sh2WriteFile(p: []const u8, data: []const u8, append: bool) void {
    const f = std.fs.cwd().createFile(p, .{ .truncate = !append }) catch return;
    defer f.close();
    f.writeAll(data) catch {};
}

pub fn main(init: std.process.Init) !void {

    const testvar: []const u8 = "";
    _ = init;

    if (std.mem.eql(u8, "x", "$(echo \"pattern\")")) { // line 3
        // declare testvar
        sh2WriteFile("/dev/null", "", false);
    }
    _ = &testvar;
}