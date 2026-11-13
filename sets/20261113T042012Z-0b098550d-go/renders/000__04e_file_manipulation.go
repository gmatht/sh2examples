package main

import (
    "fmt"
    "io"
    "os"
    "path/filepath"
    "sort"
    "strings"
    "time"
)

func main() {
    var cp_result string
    var d string
    var mkdir_result string
    var mv_result string
    var rm_result string
    var touch_result string
    var st int64

    fmt.Println("=== File Manipulation Commands ===");
    st = 0;
    d = func() string { __mknt1, __mket1 := os.MkdirTemp("", "tmp*"); if __mket1 != nil { return "" }; return __mknt1 }(); // line 10
    st = 0;
    if err := os.Chdir(d); err != nil { st = 1 } else { st = 0 }; // line 11
    if st != 0 {
        os.Exit(int(1));
    }
    if __wet2 := os.WriteFile("test_file.txt", []byte("test content\n"), 0644); __wet2 != nil { st = 1 } else { st = 0 }
    cp_result = func() string { __aost4 := ""; __cpsrct3 := "test_file.txt"; __cpdstt3 := "test_file_copy.txt"; st = 0; if __cpit3, __cpiet3 := os.Stat(__cpdstt3); __cpiet3 == nil && __cpit3.IsDir() { __cpdstt3 = filepath.Join(__cpdstt3, filepath.Base(__cpsrct3)) }; if __cpgt3, __cpet3 := os.Open(__cpsrct3); __cpet3 != nil { st = 1 } else {if __cpft3, __cpet3 := os.Create(__cpdstt3); __cpet3 != nil { st = 1 } else {if _, __cpct3 := io.Copy(__cpft3, __cpgt3); __cpct3 != nil { st = 1 }; __cpft3.Close(); if __csit3, __csiet3 := __cpgt3.Stat(); __csiet3 == nil { os.Chmod(__cpdstt3, __csit3.Mode()) }; }; __cpgt3.Close(); }; if st == 0 { __aost4 += ("Copy successful" + "\n") }; return strings.TrimRight(__aost4, "\n") }();
    st = 0;
    fmt.Println(fmt.Sprintf("Copy result: %v", cp_result)); // line 14
    st = 0;
    st = 0;
    __lsft5 := []string{}; __lsdt5 := []string{}
    for _, __lsxt5 := range []string{"test_file.txt", "test_file_copy.txt", "test_file_moved.txt"} {
        __lsit5, __lset5 := os.Stat(__lsxt5)
        if __lset5 != nil { st = 2 } else if __lsit5.IsDir() && true { __lsdt5 = append(__lsdt5, __lsxt5) } else { __lsft5 = append(__lsft5, __lsxt5) }
    }
    sort.Strings(__lsft5); sort.Strings(__lsdt5)
    for _, __lsxt5 := range __lsft5 { fmt.Println(__lsxt5) }
    for __lskt5, __lsxt5 := range __lsdt5 {
        if len(__lsft5)+len(__lsdt5) > 1 { if __lskt5 > 0 || len(__lsft5) > 0 { fmt.Println("") }; fmt.Println(__lsxt5 + ":") }
        __lsrt5, __lsret5 := os.ReadDir(__lsxt5)
        if __lsret5 != nil { st = 2; continue }
        __lsnt5 := []string{}
        for _, __lset5 := range __lsrt5 {
            __lsnmt5 := __lset5.Name()
            if strings.HasPrefix(__lsnmt5, ".") { continue }
            __lsnt5 = append(__lsnt5, __lsnmt5)
        }
        sort.Strings(__lsnt5)
        for _, __lsnmt5 := range __lsnt5 { fmt.Println(__lsnmt5) }
    }
    if st != 0 {
        fmt.Println("No test files found");
        st = 0;
    }
    mv_result = func() string { __aost7 := ""; __mvsrct6 := "test_file_copy.txt"; __mvdstt6 := "test_file_moved.txt"; st = 0; if __mvit6, __mviet6 := os.Stat(__mvdstt6); __mviet6 == nil && __mvit6.IsDir() { __mvdstt6 = filepath.Join(__mvdstt6, filepath.Base(__mvsrct6)) }; if __mvet6 := os.Rename(__mvsrct6, __mvdstt6); __mvet6 != nil {if __mvft6, __mvet6 := os.Open(__mvsrct6); __mvet6 != nil { st = 1 } else {if __mvgt6, __mvet6 := os.Create(__mvdstt6); __mvet6 != nil { st = 1 } else {if _, __mvct6 := io.Copy(__mvgt6, __mvft6); __mvct6 != nil { st = 1 } else if os.Remove(__mvsrct6) != nil { st = 1 }; __mvgt6.Close(); }; __mvft6.Close(); }; }; if st == 0 { __aost7 += ("Move successful" + "\n") }; return strings.TrimRight(__aost7, "\n") }();
    st = 0;
    fmt.Println(fmt.Sprintf("Move result: %v", mv_result)); // line 20
    st = 0;
    st = 0;
    __lsft8 := []string{}; __lsdt8 := []string{}
    for _, __lsxt8 := range []string{"test_file.txt", "test_file_copy.txt", "test_file_moved.txt"} {
        __lsit8, __lset8 := os.Stat(__lsxt8)
        if __lset8 != nil { st = 2 } else if __lsit8.IsDir() && true { __lsdt8 = append(__lsdt8, __lsxt8) } else { __lsft8 = append(__lsft8, __lsxt8) }
    }
    sort.Strings(__lsft8); sort.Strings(__lsdt8)
    for _, __lsxt8 := range __lsft8 { fmt.Println(__lsxt8) }
    for __lskt8, __lsxt8 := range __lsdt8 {
        if len(__lsft8)+len(__lsdt8) > 1 { if __lskt8 > 0 || len(__lsft8) > 0 { fmt.Println("") }; fmt.Println(__lsxt8 + ":") }
        __lsrt8, __lsret8 := os.ReadDir(__lsxt8)
        if __lsret8 != nil { st = 2; continue }
        __lsnt8 := []string{}
        for _, __lset8 := range __lsrt8 {
            __lsnmt8 := __lset8.Name()
            if strings.HasPrefix(__lsnmt8, ".") { continue }
            __lsnt8 = append(__lsnt8, __lsnmt8)
        }
        sort.Strings(__lsnt8)
        for _, __lsnmt8 := range __lsnt8 { fmt.Println(__lsnmt8) }
    }
    if st != 0 {
        fmt.Println("No test files found");
        st = 0;
    }
    rm_result = func() string { __aost10 := ""; st = 0; if err := os.Remove("test_file.txt"); err != nil { st = 1 }; if err := os.Remove("test_file_moved.txt"); err != nil { st = 1 }; if st == 0 { __aost10 += ("Remove successful" + "\n") }; return strings.TrimRight(__aost10, "\n") }();
    st = 0;
    fmt.Println(fmt.Sprintf("Remove result: %v", rm_result)); // line 26
    st = 0;
    st = 0;
    __lsft11 := []string{}; __lsdt11 := []string{}
    for _, __lsxt11 := range []string{"test_file.txt", "test_file_copy.txt", "test_file_moved.txt"} {
        __lsit11, __lset11 := os.Stat(__lsxt11)
        if __lset11 != nil { st = 2 } else if __lsit11.IsDir() && true { __lsdt11 = append(__lsdt11, __lsxt11) } else { __lsft11 = append(__lsft11, __lsxt11) }
    }
    sort.Strings(__lsft11); sort.Strings(__lsdt11)
    for _, __lsxt11 := range __lsft11 { fmt.Println(__lsxt11) }
    for __lskt11, __lsxt11 := range __lsdt11 {
        if len(__lsft11)+len(__lsdt11) > 1 { if __lskt11 > 0 || len(__lsft11) > 0 { fmt.Println("") }; fmt.Println(__lsxt11 + ":") }
        __lsrt11, __lsret11 := os.ReadDir(__lsxt11)
        if __lsret11 != nil { st = 2; continue }
        __lsnt11 := []string{}
        for _, __lset11 := range __lsrt11 {
            __lsnmt11 := __lset11.Name()
            if strings.HasPrefix(__lsnmt11, ".") { continue }
            __lsnt11 = append(__lsnt11, __lsnmt11)
        }
        sort.Strings(__lsnt11)
        for _, __lsnmt11 := range __lsnt11 { fmt.Println(__lsnmt11) }
    }
    if st != 0 {
        fmt.Println("No test files found");
        st = 0;
    }
    mkdir_result = func() string { __aost12 := ""; st = 0; if err := os.Mkdir("test_dir", 0o777); err != nil { st = 1 }; if st == 0 { __aost12 += ("Directory created" + "\n") }; return strings.TrimRight(__aost12, "\n") }();
    st = 0;
    fmt.Println(fmt.Sprintf("Mkdir result: %v", mkdir_result)); // line 32
    st = 0;
    __ttt13 := time.Now(); // line 33
    st = 0;
    if __e := os.Chtimes("test_dir/file", __ttt13, __ttt13); __e != nil { if __f, __fe := os.OpenFile("test_dir/file", os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes("test_dir/file", __ttt13, __ttt13); __e != nil { st = 1 } } }
    st = 0;
    __lsft14 := []string{}; __lsdt14 := []string{}
    for _, __lsxt14 := range []string{"test_dir"} {
        __lsit14, __lset14 := os.Stat(__lsxt14)
        if __lset14 != nil { st = 2 } else if __lsit14.IsDir() && true { __lsdt14 = append(__lsdt14, __lsxt14) } else { __lsft14 = append(__lsft14, __lsxt14) }
    }
    sort.Strings(__lsft14); sort.Strings(__lsdt14)
    for _, __lsxt14 := range __lsft14 { fmt.Println(__lsxt14) }
    for __lskt14, __lsxt14 := range __lsdt14 {
        if len(__lsft14)+len(__lsdt14) > 1 { if __lskt14 > 0 || len(__lsft14) > 0 { fmt.Println("") }; fmt.Println(__lsxt14 + ":") }
        __lsrt14, __lsret14 := os.ReadDir(__lsxt14)
        if __lsret14 != nil { st = 2; continue }
        __lsnt14 := []string{}
        for _, __lset14 := range __lsrt14 {
            __lsnmt14 := __lset14.Name()
            if strings.HasPrefix(__lsnmt14, ".") { continue }
            __lsnt14 = append(__lsnt14, __lsnmt14)
        }
        sort.Strings(__lsnt14)
        for _, __lsnmt14 := range __lsnt14 { fmt.Println(__lsnmt14) }
    }
    if st != 0 {
        fmt.Println("Directory not found");
        st = 0;
    }
    touch_result = func() string { __aost16 := ""; __ttt15 := time.Now(); st = 0; if __e := os.Chtimes("test_file.txt", __ttt15, __ttt15); __e != nil { if __f, __fe := os.OpenFile("test_file.txt", os.O_WRONLY|os.O_CREATE, 0644); __fe != nil { st = 1 } else { __f.Close(); if __e := os.Chtimes("test_file.txt", __ttt15, __ttt15); __e != nil { st = 1 } } }; if st == 0 { __aost16 += ("File touched" + "\n") }; return strings.TrimRight(__aost16, "\n") }();
    st = 0;
    fmt.Println(fmt.Sprintf("Touch result: %v", touch_result)); // line 39
    st = 0;
    st = 0; // line 42
    if err := os.Remove("test_file.txt"); err != nil && !os.IsNotExist(err) { st = 1 }
    if err := os.Remove("test_file_copy.txt"); err != nil && !os.IsNotExist(err) { st = 1 }
    if err := os.Remove("test_file_moved.txt"); err != nil && !os.IsNotExist(err) { st = 1 }
    st = 0; // line 43
    if err := os.RemoveAll("test_dir"); err != nil { st = 1 }
    if st != 0 {
        st = 0;
    }
    fmt.Println("=== File Manipulation Commands Complete ===");
    st = 0;
    if err := os.Chdir("/"); err != nil { st = 1 } else { st = 0 }; // line 46
    st = 0; // line 47
    if err := os.RemoveAll(d); err != nil { st = 1 }
    _ = st
    os.Exit(int(st));
}