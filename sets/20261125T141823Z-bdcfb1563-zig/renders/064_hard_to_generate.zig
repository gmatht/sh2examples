const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
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

fn sh2FileTest(flag: []const u8, p: []const u8) bool {
    const st = std.fs.cwd().statFile(p) catch return false;
    if (std.mem.eql(u8, flag, "-f")) return st.kind == .file;
    if (std.mem.eql(u8, flag, "-d")) return st.kind == .directory;
    if (std.mem.eql(u8, flag, "-e")) return true;
    if (std.mem.eql(u8, flag, "-s")) return st.size != 0;
    return false;
}

fn sh2WriteFile(p: []const u8, data: []const u8, append: bool) void {
    const f = std.fs.cwd().createFile(p, .{ .truncate = !append }) catch return;
    defer f.close();
    f.writeAll(data) catch {};
}

fn matchGlob(s: []const u8, pat: []const u8) bool {
    var si: usize = 0;
    var pi: usize = 0;
    var ssi: usize = 0;
    var spi: usize = 0;
    var star = false;
    while (si < s.len) {
        if (pi < pat.len) {
            const pc = pat[pi];
            if (pc == '*') {
                star = true;
                ssi = si;
                spi = pi;
                pi += 1;
                continue;
            }
            if (pc == '?' or s[si] == pc) {
                si += 1;
                pi += 1;
                continue;
            }
        }
        if (star) { si = ssi + 1; ssi = si; pi = spi + 1; } else return false;
    }
    while (pi < pat.len and pat[pi] == '*') { pi += 1; }
    return pi == pat.len;
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

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
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

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

fn sh2Shopt() []const u8 {
    sh2TODO("shopt");
    return "";
}

fn sh2TestStub() bool {
    sh2TODO("test");
    return false;
}

pub fn main(init: std.process.Init) !void {

    var __ps_tmp0: []const u8 = "";
    var __ps_tmp1: []const u8 = "";
    var __ps_tmp2: []const u8 = "";
    var __ps_tmp3: []const u8 = "";
    var __ps_tmp4: []const u8 = "";
    var __ps_tmp5: []const u8 = "";
    var basename: []const u8 = "";
    var config__host__: []const u8 = "";
    var config__port__: []const u8 = "";
    var config__user__: []const u8 = "";
    var extension: []const u8 = "";
    var file: []const u8 = "";
    var filename: []const u8 = "";
    var first_half: [1024]i64 = undefined;
    var first_half_len: usize = 0;
    var i: []const u8 = "";
    var input: []const u8 = "";
    var j: []const u8 = "";
    var last_half: [1024]i64 = undefined;
    var last_half_len: usize = 0;
    var message: []const u8 = "";
    var middle: [1024]i64 = undefined;
    var middle_len: usize = 0;
    var name: []const u8 = "";
    var numbers: [1024]i64 = undefined;
    var numbers_len: usize = 0;
    var result: []const u8 = "";
    var sanitized: []const u8 = "";
    const uid: []const u8 = "";
    var uppercase: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    __ps_tmp1 = sh2Capture();
    __ps_tmp0 = sh2Capture();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    sh2WriteFile(__ps_tmp0, "", false);
    _ = sh2Run(&[_][]const u8{"sort", __ps_tmp0});
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    sh2WriteFile(__ps_tmp1, "", false);
    __ps_tmp3 = sh2Capture();
    __ps_tmp2 = sh2Capture();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    sh2WriteFile(__ps_tmp2, "", false);
    _ = sh2Run(&[_][]const u8{"sort", __ps_tmp2});
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    sh2WriteFile(__ps_tmp3, "", false);
    _ = sh2Run(&[_][]const u8{"diff", __ps_tmp1, __ps_tmp3});
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    try stdout.print("{s}\n", .{"Files:  file_a_1..10.txt file_a_1..10.log file_a_1..10.dat file_a_20.txt file_a_20.log file_a_20.dat file_a_30..40.txt file_a_30..40.log file_a_30..40.dat file_b_1..10.txt file_b_1..10.log file_b_1..10.dat file_b_20.txt file_b_20.log file_b_20.dat file_b_30..40.txt file_b_30..40.log file_b_30..40.dat file_c_1..10.txt file_c_1..10.log file_c_1..10.dat file_c_20.txt file_c_20.log file_c_20.dat file_c_30..40.txt file_c_30..40.log file_c_30..40.dat file_d_1..10.txt file_d_1..10.log file_d_1..10.dat file_d_20.txt file_d_20.log file_d_20.dat file_d_30..40.txt file_d_30..40.log file_d_30..40.dat file_e_1..10.txt file_e_1..10.log file_e_1..10.dat file_e_20.txt file_e_20.log file_e_20.dat file_e_30..40.txt file_e_30..40.log file_e_30..40.dat file_f_1..10.txt file_f_1..10.log file_f_1..10.dat file_f_20.txt file_f_20.log file_f_20.dat file_f_30..40.txt file_f_30..40.log file_f_30..40.dat file_g_1..10.txt file_g_1..10.log file_g_1..10.dat file_g_20.txt file_g_20.log file_g_20.dat file_g_30..40.txt file_g_30..40.log file_g_30..40.dat file_h_1..10.txt file_h_1..10.log file_h_1..10.dat file_h_20.txt file_h_20.log file_h_20.dat file_h_30..40.txt file_h_30..40.log file_h_30..40.dat file_i_1..10.txt file_i_1..10.log file_i_1..10.dat file_i_20.txt file_i_20.log file_i_20.dat file_i_30..40.txt file_i_30..40.log file_i_30..40.dat file_j_1..10.txt file_j_1..10.log file_j_1..10.dat file_j_20.txt file_j_20.log file_j_20.dat file_j_30..40.txt file_j_30..40.log file_j_30..40.dat file_k_1..10.txt file_k_1..10.log file_k_1..10.dat file_k_20.txt file_k_20.log file_k_20.dat file_k_30..40.txt file_k_30..40.log file_k_30..40.dat file_l_1..10.txt file_l_1..10.log file_l_1..10.dat file_l_20.txt file_l_20.log file_l_20.dat file_l_30..40.txt file_l_30..40.log file_l_30..40.dat file_m_1..10.txt file_m_1..10.log file_m_1..10.dat file_m_20.txt file_m_20.log file_m_20.dat file_m_30..40.txt file_m_30..40.log file_m_30..40.dat file_n_1..10.txt file_n_1..10.log file_n_1..10.dat file_n_20.txt file_n_20.log file_n_20.dat file_n_30..40.txt file_n_30..40.log file_n_30..40.dat file_o_1..10.txt file_o_1..10.log file_o_1..10.dat file_o_20.txt file_o_20.log file_o_20.dat file_o_30..40.txt file_o_30..40.log file_o_30..40.dat file_p_1..10.txt file_p_1..10.log file_p_1..10.dat file_p_20.txt file_p_20.log file_p_20.dat file_p_30..40.txt file_p_30..40.log file_p_30..40.dat file_q_1..10.txt file_q_1..10.log file_q_1..10.dat file_q_20.txt file_q_20.log file_q_20.dat file_q_30..40.txt file_q_30..40.log file_q_30..40.dat file_r_1..10.txt file_r_1..10.log file_r_1..10.dat file_r_20.txt file_r_20.log file_r_20.dat file_r_30..40.txt file_r_30..40.log file_r_30..40.dat file_s_1..10.txt file_s_1..10.log file_s_1..10.dat file_s_20.txt file_s_20.log file_s_20.dat file_s_30..40.txt file_s_30..40.log file_s_30..40.dat file_t_1..10.txt file_t_1..10.log file_t_1..10.dat file_t_20.txt file_t_20.log file_t_20.dat file_t_30..40.txt file_t_30..40.log file_t_30..40.dat file_u_1..10.txt file_u_1..10.log file_u_1..10.dat file_u_20.txt file_u_20.log file_u_20.dat file_u_30..40.txt file_u_30..40.log file_u_30..40.dat file_v_1..10.txt file_v_1..10.log file_v_1..10.dat file_v_20.txt file_v_20.log file_v_20.dat file_v_30..40.txt file_v_30..40.log file_v_30..40.dat file_w_1..10.txt file_w_1..10.log file_w_1..10.dat file_w_20.txt file_w_20.log file_w_20.dat file_w_30..40.txt file_w_30..40.log file_w_30..40.dat file_x_1..10.txt file_x_1..10.log file_x_1..10.dat file_x_20.txt file_x_20.log file_x_20.dat file_x_30..40.txt file_x_30..40.log file_x_30..40.dat file_y_1..10.txt file_y_1..10.log file_y_1..10.dat file_y_20.txt file_y_20.log file_y_20.dat file_y_30..40.txt file_y_30..40.log file_y_30..40.dat file_z_1..10.txt file_z_1..10.log file_z_1..10.dat file_z_20.txt file_z_20.log file_z_20.dat file_z_30..40.txt file_z_30..40.log file_z_30..40.dat"});
    try stdout.flush();
    name = "John Doe";
    try stdout.print("{s}{s}\n", .{"Hello ", "John_Doe"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"Length: ", "8"});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"First: ", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join // line 17
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"Last: ", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): shopt → sh2.shopt // line 20
    // TODO(unsupported): shopt → sh2.shopt
    _ = sh2Shopt();
    // TODO(unsupported): shopt → sh2.shopt // line 21
    // TODO(unsupported): shopt → sh2.shopt
    _ = sh2Shopt();
    if (matchGlob(sh2Env("1"), "[a-z]*")) { // line 24
        try stdout.print("{s}\n", .{"Lowercase start"});
    try stdout.flush();
    } else if (matchGlob(sh2Env("1"), "[A-Z]*")) {
        try stdout.print("{s}\n", .{"Uppercase start"});
    try stdout.flush();
    } else if (matchGlob(sh2Env("1"), "[0-9]*")) {
        try stdout.print("{s}\n", .{"Number start"});
    try stdout.flush();
    } else if (matchGlob(sh2Env("1"), "?")) {
        try stdout.print("{s}\n", .{"Single character"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"Something else"});
    try stdout.flush();
    }
    i = sh2IntStr(2);
    j = sh2IntStr(((sh2ToInt(i)) + (sh2ToInt(i)))); // line 34
    try stdout.print("{s}{s}{s}{s}\n", .{"i=", i, ", j=", j});
    try stdout.flush(); // line 35
    _ = sh2Run(&[_][]const u8{"declare", "-A", "config"}); // line 38
    config__user__ = "admin";
    config__host__ = "localhost";
    config__port__ = "8080";
    _ = sh2Run(&[_][]const u8{"cat"}); // line 44
    sh2WriteFile("config.txt", "", false);
    __ps_tmp4 = sh2Capture();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    sh2WriteFile(__ps_tmp4, "", false);
    __ps_tmp5 = sh2Capture();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    sh2WriteFile(__ps_tmp5, "", false);
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    if ((std.mem.eql(u8, sh2IntStr(1), sh2GetVar()) and sh2Test())) {
        try stdout.print("{s}\n", .{"Valid input"});
    try stdout.flush();
    }
    _ = sh2Run(&[_][]const u8{"mkdir", "-p", "project/src/main/java", "project/src/main/resources", "project/src/test/java", "project/src/test/resources", "project/docs/api", "project/docs/user", "project/build/classes", "project/build/lib"});
    filename = "my_file.txt";
    // TODO(unsupported): param → sh2.param // line 79
    // TODO(unsupported): param → sh2.param
    basename = sh2Param();
    // TODO(unsupported): param → sh2.param // line 80
    // TODO(unsupported): param → sh2.param
    extension = sh2Param();
    uppercase = "MY_FILE.TXT";
    result = sh2Capture(); // line 85
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    while (sh2Truthy(sh2Exec())) {
        if (((sh2ToInt(uid) > 1000) and !std.mem.eql(u8, sh2GetVar(), "/bin/false"))) {
            if (std.mem.eql(u8, sh2GetVar(), "~^/home/")) {
                try stdout.print("{s}{s}{s}{s}{s}{s}\n", .{"User: ", sh2Env("user"), " (UID: ", uid, ") - ", sh2Env("home")});
    try stdout.flush();
            }
        }
    }
    numbers_len = 0; // line 115
    numbers[numbers_len] = 1;
    numbers_len += 1;
    numbers[numbers_len] = 2;
    numbers_len += 1;
    numbers[numbers_len] = 3;
    numbers_len += 1;
    numbers[numbers_len] = 4;
    numbers_len += 1;
    numbers[numbers_len] = 5;
    numbers_len += 1;
    numbers[numbers_len] = 6;
    numbers_len += 1;
    numbers[numbers_len] = 7;
    numbers_len += 1;
    numbers[numbers_len] = 8;
    numbers_len += 1;
    numbers[numbers_len] = 9;
    numbers_len += 1;
    numbers[numbers_len] = 10;
    numbers_len += 1;
    middle_len = 0; // line 116
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    middle[middle_len] = sh2ToInt(sh2Join());
    middle_len += 1;
    first_half_len = 0; // line 117
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    first_half[first_half_len] = sh2ToInt(sh2Join());
    first_half_len += 1;
    last_half_len = 0; // line 118
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    last_half[last_half_len] = sh2ToInt(sh2Join());
    last_half_len += 1;
    {
        const sh2_items0 = [_][]const u8{"\x01SH2GLOB\x01*.txt", "\x01SH2GLOB\x01*.log", "\x01SH2GLOB\x01*.dat"};
        for (sh2_items0[0..]) |sh2_v1| {
            file = sh2_v1;
            if (matchGlob(sh2IntStr(file), "*.txt") or matchGlob(sh2IntStr(file), "*.log")) {
                try stdout.print("{s}{s}\n", .{"Text file: ", file});
    try stdout.flush();
            } else if (matchGlob(sh2IntStr(file), "*.dat")) {
                try stdout.print("{s}{s}\n", .{"Data file: ", file});
    try stdout.flush();
            } else {
                try stdout.print("{s}{s}\n", .{"Other file: ", file});
    try stdout.flush();
            }
        }
    }
    _ = sh2Run(&[_][]const u8{"export", "DEBUG=1"}); // line 130
    _ = sh2Run(&[_][]const u8{"export", "LOG_LEVEL=verbose"});
    _ = sh2Run(&[_][]const u8{"unset", "DEBUG"});
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}\n", .{"Inner: LOG_LEVEL=", sh2Env("LOG_LEVEL"), ", DEBUG=", sh2Param()});
    try stdout.flush();
    try stdout.print("{s}{s}{s}{s}\n", .{"Outer: LOG_LEVEL=", sh2Env("LOG_LEVEL"), ", DEBUG=", sh2Env("DEBUG")});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 143
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    message = std.fmt.allocPrint(std.heap.page_allocator, "Hello {s} from {s}", .{sh2Param(), sh2Param()}) catch "";
    try stdout.print("{s}\n", .{message});
    try stdout.flush(); // line 144
    _ = sh2Run(&[_][]const u8{"trap", "echo \"Error on line $LINENO\"; exit 1", "ERR"}); // line 159
    _ = sh2Run(&[_][]const u8{"trap", "echo \"Cleaning up...\"; rm -f /tmp/064_hard_temp_*", "EXIT"}); // line 160
    // TODO(unsupported): param → sh2.param // line 163
    // TODO(unsupported): param → sh2.param
    input = sh2Param();
    // TODO(unsupported): param → sh2.param // line 164
    // TODO(unsupported): param → sh2.param
    sanitized = sh2Param();
    // TODO(unsupported): param → sh2.param // line 165
    // TODO(unsupported): param → sh2.param
    uppercase = sh2Param();
    try stdout.print("{s}{s}{s}{s}{s}{s}{s}\n", .{"Input: '", input, "' -> Sanitized: '", sanitized, "' -> Uppercase: '", uppercase, "'"});
    try stdout.flush(); // line 166
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = ((sh2FileTest("-f", sh2IntStr(1)) and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (((sh2FileTest("-d", sh2IntStr(2)) and sh2Truthy(sh2Exec())) and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
    try stdout.print("{s}\n", .{"Script completed successfully!"});
    try stdout.flush();
    _ = &basename;
    _ = &config__host__;
    _ = &config__port__;
    _ = &config__user__;
    _ = &extension;
    _ = &filename;
    _ = &first_half;
    _ = &last_half;
    _ = &middle;
    _ = &name;
    _ = &numbers;
    _ = &result;
}
// 62 construct(s) lowered to TODO markers