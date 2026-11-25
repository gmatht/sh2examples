const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2ArrayIndex() []const u8 {
    sh2TODO("arrayIndex");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var FilesystemOptions: [1024]i64 = undefined;
    var FilesystemOptions_len: usize = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    FilesystemOptions_len = 0; // line 2
    FilesystemOptions[FilesystemOptions_len] = sh2ToInt("a");
    FilesystemOptions_len += 1;
    FilesystemOptions[FilesystemOptions_len] = sh2ToInt("b");
    FilesystemOptions_len += 1;
    FilesystemOptions[FilesystemOptions_len] = sh2ToInt("c");
    FilesystemOptions_len += 1;
    // TODO(unsupported): arrayIndex → sh2.arrayIndex
    // TODO(unsupported): arrayIndex → sh2.arrayIndex
    try stdout.print("{s}\n", .{sh2ArrayIndex()});
    try stdout.flush();
    _ = &FilesystemOptions;
}
// 2 construct(s) lowered to TODO markers