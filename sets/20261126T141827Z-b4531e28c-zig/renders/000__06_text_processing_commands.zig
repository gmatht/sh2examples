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

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var awk_result: []const u8 = "";
    var comm_result: []const u8 = "";
    var cut_result: []const u8 = "";
    var diff_result: []const u8 = "";
    var file_content: []const u8 = "";
    var grep_result: []const u8 = "";
    var head_result: []const u8 = "";
    var line_count: []const u8 = "";
    var paste_result: []const u8 = "";
    var sed_result: []const u8 = "";
    var sort_result: []const u8 = "";
    var tail_result: []const u8 = "";
    var tr_result: []const u8 = "";
    var uniq_result: []const u8 = "";
    var word_count: []const u8 = "";
    var xargs_result: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    try stdout.print("{s}\n", .{"=== Text Processing Commands ==="});
    try stdout.flush();
    file_content = sh2Capture(); // line 10
    try stdout.print("{s}\n", .{"First 5 lines of main.rs:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{file_content});
    try stdout.flush(); // line 12
    grep_result = sh2Capture();
    try stdout.print("{s}\n", .{"Lines containing 'fn':"});
    try stdout.flush();
    try stdout.print("{s}\n", .{grep_result});
    try stdout.flush(); // line 18
    sed_result = sh2Capture();
    try stdout.print("{s}{s}\n", .{"Sed result: ", sed_result});
    try stdout.flush(); // line 23
    awk_result = sh2Capture();
    try stdout.print("{s}{s}\n", .{"Awk sum result: ", awk_result});
    try stdout.flush(); // line 28
    sort_result = sh2Capture();
    try stdout.print("{s}\n", .{"Sorted words:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{sort_result});
    try stdout.flush(); // line 34
    uniq_result = sh2Capture();
    try stdout.print("{s}\n", .{"Unique words:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{uniq_result});
    try stdout.flush(); // line 40
    word_count = sh2Capture();
    line_count = sh2Capture();
    try stdout.print("{s}{s}\n", .{"Word count: ", word_count});
    try stdout.flush(); // line 46
    try stdout.print("{s}{s}\n", .{"Line count: ", line_count});
    try stdout.flush(); // line 47
    head_result = sh2Capture(); // line 51
    try stdout.print("{s}{s}\n", .{"First 3 numbers: ", head_result});
    try stdout.flush(); // line 52
    tail_result = sh2Capture(); // line 56
    try stdout.print("{s}{s}\n", .{"Last 3 numbers: ", tail_result});
    try stdout.flush(); // line 57
    cut_result = sh2Capture();
    try stdout.print("{s}{s}\n", .{"Second field: ", cut_result});
    try stdout.flush(); // line 62
    try std.fs.cwd().writeFile(.{ .sub_path = "temp1.txt", .data = "1\n2\n3\n" });
    try std.fs.cwd().writeFile(.{ .sub_path = "temp2.txt", .data = "a\nb\nc\n" });
    paste_result = sh2Capture(); // line 68
    try stdout.print("{s}\n", .{"Pasted columns:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{paste_result});
    try stdout.flush(); // line 70
    _ = sh2Run(&[_][]const u8{"rm", "-f", "temp1.txt", "temp2.txt"}); // line 71
    try std.fs.cwd().writeFile(.{ .sub_path = "file1.txt", .data = "apple\nbanana\ncherry\n" });
    try std.fs.cwd().writeFile(.{ .sub_path = "file2.txt", .data = "banana\ncherry\ndate\n" });
    comm_result = sh2Capture(); // line 77
    try stdout.print("{s}\n", .{"Common lines:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{comm_result});
    try stdout.flush(); // line 79
    diff_result = sh2Capture(); // line 83
    try stdout.print("{s}\n", .{"File differences:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{diff_result});
    try stdout.flush(); // line 85
    tr_result = sh2Capture();
    try stdout.print("{s}{s}\n", .{"Lowercase: ", tr_result});
    try stdout.flush(); // line 90
    xargs_result = sh2Capture();
    try stdout.print("{s}\n", .{"Xargs result:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{xargs_result});
    try stdout.flush(); // line 96
    _ = sh2Run(&[_][]const u8{"rm", "-f", "file1.txt", "file2.txt"}); // line 99
}