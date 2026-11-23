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

fn sh2ListVar() []const u8 {
    sh2TODO("listVar");
    return "";
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}


fn process_files(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    _ = __args;
    var count: i64 = 0;
    var file: []const u8 = "";
    // TODO(unsupported): listVar → sh2.listVar
    // TODO(unsupported): listVar → sh2.listVar
    _ = sh2Run(&[_][]const u8{"local", "-a", [_] []const u8{sh2ListVar()}});
    count = 0;
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    {
        const sh2_items0 = [_][]const u8{sh2Param()};
        for (sh2_items0[0..]) |sh2_v1| {
            file = sh2_v1;
            if (sh2FileTest("-f", file)) {
                count += 1;
                try stdout.print("{s}{s}\n", .{"Processing: ", file});
    try stdout.flush();
            }
        }
    }
    try stdout.print("{s}{d}\n", .{"Total files processed: ", count});
    try stdout.flush();
}
pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"process_files", "file1.txt", "file2.txt", "nonexistent.txt"});
}
// 4 construct(s) lowered to TODO markers