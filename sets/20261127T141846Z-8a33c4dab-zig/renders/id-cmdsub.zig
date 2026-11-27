const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

fn sh2WriteFile(p: []const u8, data: []const u8, append: bool) void {
    const f = std.fs.cwd().createFile(p, .{ .truncate = !append }) catch return;
    defer f.close();
    f.writeAll(data) catch {};
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

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}


fn capture(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    var se: []const u8 = "";
    var so: []const u8 = "";
    var tmp_stderr: []const u8 = "";
    var tmp_stdout: []const u8 = "";
    _ = sh2Run(&[_][]const u8{"local", "label=", (if (__args.len >= 1) __args[1 - 1] else "")});
    _ = sh2Run(&[_][]const u8{"shift"});
    // declare tmp_stdout
    // declare tmp_stderr
    tmp_stdout = sh2Capture();
    tmp_stderr = sh2Capture();
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    sh2WriteFile(sh2IntStr(tmp_stdout), "", false);
    _ = sh2Run(&[_][]const u8{"local", "ec=", sh2Env("?")});
    // declare so
    so = sh2Capture();
    // declare se
    se = sh2Capture();
    _ = sh2Run(&[_][]const u8{"rm", "-f", sh2IntStr(tmp_stdout), sh2IntStr(tmp_stderr)});
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}\n", .{"--- [", sh2Param(), "] ---"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"  cmd     : ", sh2Env("*")});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"  exitcode: ", sh2Param()});
    try stdout.flush();
    if (!std.mem.eql(u8, so, "")) {
        // TODO(unsupported): param → sh2.param
        // TODO(unsupported): param → sh2.param
        try stdout.print("{s}{s}\n", .{"  stdout  : ", sh2Param()});
    try stdout.flush();
    }
    if (!std.mem.eql(u8, se, "")) {
        // TODO(unsupported): param → sh2.param
        // TODO(unsupported): param → sh2.param
        try stdout.print("{s}{s}\n", .{"  stderr  : ", sh2Param()});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    return sh2Env("ec");
}
pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    _ = sh2Run(&[_][]const u8{"set", "-u"}); // line 13
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 1: 'id' (default \xe2\x80\x94 current process)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "01-default", "id"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 2: id -u  (effective user ID, numeric)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "02-u", "id", "-u"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 3: id -g  (effective group ID, numeric)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "03-g", "id", "-g"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 4: id -G  (all group IDs, numeric)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "04-G", "id", "-G"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 5: id -un  (effective user name)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "05-un", "id", "-un"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 6: id -gn  (effective group name)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "06-gn", "id", "-gn"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 7: id -Gn  (all group names)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "07-Gn", "id", "-Gn"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 8: id -ru  (real user ID, numeric)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "08-ru", "id", "-ru"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 9: id -rg  (real group ID, numeric)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "09-rg", "id", "-rg"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 10: id -rG  (real group IDs, numeric)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "10-rG", "id", "-rG"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 11: id -run  (real user name)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "11-run", "id", "-run"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 12: id -rgn  (real group name)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "12-rgn", "id", "-rgn"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 13: id -rGn  (real group names)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "13-rGn", "id", "-rGn"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 14: id -u -n  (separate options, equivalent to -un)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "14-u-n", "id", "-u", "-n"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 15: id -r -u  (separate options, equivalent to -ru)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "15-r-u", "id", "-r", "-u"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 16: id -G -z  (group IDs delimited by NUL)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "16-Gz", "bash", "-c", "id -G -z | cat -v"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 17: id -Gn -z  (group names delimited by NUL)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "17-Gnz", "bash", "-c", "id -Gn -z | cat -v"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 18: id -un -z  (user name with NUL terminator)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "18-unz", "bash", "-c", "id -un -z | cat -v"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 19: id -Z  (security context \xe2\x80\x94 may not be available)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "19-Z", "id", "-Z"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 20: id root  (default output for user 'root')"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "20-root", "id", "root"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 21: id -u root  (numeric UID of root)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "21-u-root", "id", "-u", "root"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 22: id -g root  (numeric GID of root)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "22-g-root", "id", "-g", "root"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 23: id -G root  (all group IDs of root, numeric)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "23-G-root", "id", "-G", "root"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 24: id -un root  (user name of root)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "24-un-root", "id", "-un", "root"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 25: id -ru root  (real user ID of root, numeric)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "25-ru-root", "id", "-ru", "root"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 26: id -a  (compatibility flag \xe2\x80\x94 same as default)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "26-a", "id", "-a"});
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    try stdout.print("{s}\n", .{"All id demo sections completed."});
    try stdout.flush();
    std.process.exit(@intCast(0));
}
// 10 construct(s) lowered to TODO markers