package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "strings"
)

func s2s(x any) string {
    if x == nil { return "" }
    return fmt.Sprintf("%v", x)
}

func runCmd(c *exec.Cmd) int64 {
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    if err := c.Run(); err != nil {
        if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
        return 1
    }
    return 0
}

func capCmd(c *exec.Cmd) string {
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func capRun(script string, env []string) string {
    c := exec.Command("bash", "-c", script)
    c.Env = append(os.Environ(), env...)
    c.Stderr = io.Discard
    out, err := c.Output()
    if err != nil { return "" }
    return strings.TrimRight(string(out), "\n")
}

func main() {
    var awk_result string
    var comm_result string
    var cut_result string
    var diff_result string
    var file_content string
    var grep_result string
    var head_result string
    var line_count string
    var paste_result string
    var sed_result string
    var sort_result string
    var tail_result string
    var tr_result string
    var uniq_result string
    var word_count string
    var xargs_result string
    var st int64

    fmt.Println("=== Text Processing Commands ===");
    file_content = s2s(capRun("'cat' 'src/main.rs' | 'head' '-5'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)})); // line 10
    fmt.Println("First 5 lines of main.rs:");
    fmt.Println(fmt.Sprintf("%v", file_content)); // line 12
    grep_result = s2s(capCmd(exec.Command("grep", "-n", "fn", "src/main.rs")));
    fmt.Println("Lines containing 'fn':");
    fmt.Println(fmt.Sprintf("%v", grep_result)); // line 18
    sed_result = s2s(capRun("'echo' 'Hello World' | 'sed' 's/World/Universe/'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)}));
    fmt.Println(fmt.Sprintf("Sed result: %v", sed_result)); // line 23
    awk_result = s2s(capRun("'echo' '1 2 3 4 5' | 'awk' '{print $1 + $2}'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)}));
    fmt.Println(fmt.Sprintf("Awk sum result: %v", awk_result)); // line 28
    sort_result = s2s(capRun("'echo' '-e' 'zebra\\napple\\nbanana' | 'sort'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)}));
    fmt.Println("Sorted words:");
    fmt.Println(fmt.Sprintf("%v", sort_result)); // line 34
    uniq_result = s2s(capRun("'echo' '-e' 'apple\\napple\\nbanana\\nbanana\\ncherry' | 'uniq'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)}));
    fmt.Println("Unique words:");
    fmt.Println(fmt.Sprintf("%v", uniq_result)); // line 40
    word_count = s2s(capRun("'echo' 'Hello World' | 'wc' '-w'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)}));
    line_count = s2s(capRun("'echo' '-e' 'line1\\nline2\\nline3' | 'wc' '-l'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)}));
    fmt.Println(fmt.Sprintf("Word count: %v", word_count)); // line 46
    fmt.Println(fmt.Sprintf("Line count: %v", line_count)); // line 47
    head_result = s2s(capRun("'seq' '1' '10' | 'head' '-3'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)})); // line 51
    fmt.Println(fmt.Sprintf("First 3 numbers: %v", head_result)); // line 52
    tail_result = s2s(capRun("'seq' '1' '10' | 'tail' '-3'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)})); // line 56
    fmt.Println(fmt.Sprintf("Last 3 numbers: %v", tail_result)); // line 57
    cut_result = s2s(capRun("'echo' 'apple:banana:cherry' | 'cut' '-d' ':' '-f' '2'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)}));
    fmt.Println(fmt.Sprintf("Second field: %v", cut_result)); // line 62
    os.WriteFile("temp1.txt", []byte("1\n2\n3\n"), 0644);
    os.WriteFile("temp2.txt", []byte("a\nb\nc\n"), 0644);
    paste_result = s2s(capRun("'paste' 'temp1.txt' 'temp2.txt' | 'sed' 's/\\t/ /g'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)})); // line 68
    fmt.Println("Pasted columns:");
    fmt.Println(fmt.Sprintf("%v", paste_result)); // line 70
    st = runCmd(exec.Command("rm", "-f", "temp1.txt", "temp2.txt")); // line 71
    os.WriteFile("file1.txt", []byte("apple\nbanana\ncherry\n"), 0644);
    os.WriteFile("file2.txt", []byte("banana\ncherry\ndate\n"), 0644);
    comm_result = s2s(capCmd(exec.Command("comm", "-12", "file1.txt", "file2.txt"))); // line 77
    st = 0;
    fmt.Println("Common lines:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", comm_result)); // line 79
    st = 0;
    diff_result = s2s(capCmd(exec.Command("diff", "file1.txt", "file2.txt"))); // line 83
    st = 0;
    fmt.Println("File differences:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", diff_result)); // line 85
    st = 0;
    tr_result = s2s(capRun("'echo' 'HELLO WORLD' | 'tr' 'A-Z' 'a-z'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)}));
    st = 0;
    fmt.Println(fmt.Sprintf("Lowercase: %v", tr_result)); // line 90
    st = 0;
    xargs_result = s2s(capRun("'echo' '1 2 3' | 'xargs' '-n' '1' 'echo' 'Number:'", []string{"awk_result=" + s2s(awk_result), "comm_result=" + s2s(comm_result), "cut_result=" + s2s(cut_result), "diff_result=" + s2s(diff_result), "file_content=" + s2s(file_content), "grep_result=" + s2s(grep_result), "head_result=" + s2s(head_result), "line_count=" + s2s(line_count), "paste_result=" + s2s(paste_result), "sed_result=" + s2s(sed_result), "sort_result=" + s2s(sort_result), "tail_result=" + s2s(tail_result), "tr_result=" + s2s(tr_result), "uniq_result=" + s2s(uniq_result), "word_count=" + s2s(word_count), "xargs_result=" + s2s(xargs_result)}));
    st = 0;
    fmt.Println("Xargs result:");
    st = 0;
    fmt.Println(fmt.Sprintf("%v", xargs_result)); // line 96
    st = 0;
    st = runCmd(exec.Command("rm", "-f", "file1.txt", "file2.txt")); // line 99
    _ = st
    return;
}