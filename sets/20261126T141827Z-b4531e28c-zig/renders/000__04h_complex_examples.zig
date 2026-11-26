const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
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

fn sh2Join() []const u8 {
    sh2TODO("join");
    return "";
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}


fn get_file_size(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    _ = __args;
    _ = sh2Run(&[_][]const u8{"local", "file=$1"});
    // TODO(unsupported): capture → sh2.capture
    // TODO(unsupported): capture → sh2.capture
    _ = sh2Run(&[_][]const u8{"local", "size=", sh2Capture()});
    try stdout.print("{s}{s}{s}{s}{s}\n", .{"File ", file, " has ", sh2Env("size"), " bytes"});
    try stdout.flush();
}
pub fn main(init: std.process.Init) !void {

    var count: []const u8 = "";
    var d: []const u8 = "";
    var file: []const u8 = "";
    var files: [1024]i64 = undefined;
    var files_len: usize = 0;
    var here_string_result: []const u8 = "";
    var nested_result: []const u8 = "";
    var perl_result: []const u8 = "";
    var process_result: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    d = sh2Capture(); // line 4
    // TODO(unsupported): exec → sh2.exec // line 5
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or sh2Truthy(sh2Exec()));
    try stdout.print("{s}\n", .{"=== Complex Backtick Examples ==="});
    try stdout.flush();
    nested_result = sh2Capture(); // line 11
    try stdout.print("{s}{s}\n", .{"Nested backticks: ", nested_result});
    try stdout.flush(); // line 12
    count = sh2Capture(); // line 15
    try stdout.print("{s}{s}\n", .{"File count: ", count});
    try stdout.flush(); // line 16
    if (std.mem.eql(u8, "root", "root")) {
        try stdout.print("{s}\n", .{"Running as root"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"Not running as root"});
    try stdout.flush();
    }
    if (false) {
        try stdout.print("{s}\n", .{"Running on Linux"});
    try stdout.flush();
    } else if (true) {
        try stdout.print("{s}\n", .{"Running on macOS"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"Running on other system"});
    try stdout.flush();
    }
    _ = sh2Run(&[_][]const u8{"get_file_size", "000__01_file_directory_operations.sh"}); // line 47
    files_len = 0; // line 50
    // TODO(unsupported): capture → sh2.capture
    // TODO(unsupported): capture → sh2.capture
    files[files_len] = sh2ToInt(sh2Capture());
    files_len += 1;
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"Shell scripts found: ", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 52
    // TODO(unsupported): param → sh2.param
    {
        const sh2_items0 = [_][]const u8{sh2Param()};
        for (sh2_items0[0..]) |sh2_v1| {
            file = sh2_v1;
            try stdout.print("{s}{s}\n", .{"  - ", file});
    try stdout.flush();
        }
    }
    try std.fs.cwd().writeFile(.{ .sub_path = "file1.txt", .data = "apple\nbanana\ncherry\n" });
    try std.fs.cwd().writeFile(.{ .sub_path = "file2.txt", .data = "banana\ncherry\ndate\n" });
    process_result = sh2Capture();
    try stdout.print("{s}\n", .{"Process substitution result:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{process_result});
    try stdout.flush(); // line 61
    here_string_result = sh2Capture(); // line 64
    try stdout.print("{s}{s}\n", .{"Here string result: ", here_string_result});
    try stdout.flush(); // line 65
    perl_result = sh2Capture(); // line 69
    try stdout.print("{s}{s}\n", .{"Perl result: ", perl_result});
    try stdout.flush(); // line 70
    _ = sh2Run(&[_][]const u8{"rm", "-f", "file1.txt", "file2.txt"}); // line 73
    try stdout.print("{s}\n", .{"=== Complex Backtick Examples Complete ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"cd", "/"}); // line 76
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", d}); // line 77
    _ = &files;
}
// 12 construct(s) lowered to TODO markers