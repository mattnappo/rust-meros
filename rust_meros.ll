; ModuleID = 's4daksrfyqmy5q5'
source_filename = "s4daksrfyqmy5q5"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"rust_meros::primitives::file::FileError" = type { [0 x i64], i64, [3 x i64] }
%"core::fmt::Formatter" = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i64], { {}*, [3 x i64]* }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }
%"rust_meros::primitives::file::File" = type { [0 x i64], %"alloc::string::String", [0 x i8], %"rust_meros::primitives::file::FileID", [0 x i8] }
%"alloc::string::String" = type { [0 x i64], %"alloc::vec::Vec<u8>", [0 x i64] }
%"alloc::vec::Vec<u8>" = type { [0 x i64], { i8*, i64 }, [0 x i64], i64, [0 x i64] }
%"rust_meros::primitives::file::FileID" = type { [0 x i8], [32 x i8], [0 x i8] }
%"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>" = type { [0 x i64], i64, [7 x i64] }
%"core::panic::Location" = type { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }
%"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>::Err" = type { [1 x i64], %"rust_meros::primitives::file::FileError", [0 x i64] }
%"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>::Ok" = type { [1 x i64], %"rust_meros::primitives::file::File", [0 x i64] }
%"std::path::Path" = type { [0 x i8], %"std::ffi::os_str::OsStr" }
%"std::ffi::os_str::OsStr" = type { [0 x i8], %"std::sys_common::os_str_bytes::Slice" }
%"std::sys_common::os_str_bytes::Slice" = type { [0 x i8], [0 x i8] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@vtable.0 = private unnamed_addr constant { void (i8**)*, i64, i64, i32 (i8**)*, i32 (i8**)*, i32 (i8**)* } { void (i8**)* @_ZN4core3ptr13drop_in_place17hf761a04b7e1ae360E, i64 8, i64 8, i32 (i8**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9037d04980ce8d14E", i32 (i8**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9037d04980ce8d14E", i32 (i8**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h1aa824039daa9e63E" }, align 8, !dbg !0
@alloc22 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Result::unwrap()` on an `Err` value" }>, align 1
@vtable.1 = private unnamed_addr constant { void (%"rust_meros::primitives::file::FileError"*)*, i64, i64, i1 (%"rust_meros::primitives::file::FileError"*, %"core::fmt::Formatter"*)* } { void (%"rust_meros::primitives::file::FileError"*)* @_ZN4core3ptr13drop_in_place17h509d85da10b4e18bE, i64 32, i64 8, i1 (%"rust_meros::primitives::file::FileError"*, %"core::fmt::Formatter"*)* @"_ZN76_$LT$rust_meros..primitives..file..FileError$u20$as$u20$core..fmt..Debug$GT$3fmt17h6cd85552ea2d3842E" }, align 8, !dbg !14
@alloc23 = private unnamed_addr constant <{ [10 x i8] }> <{ [10 x i8] c"./testfile" }>, align 1
@alloc24 = private unnamed_addr constant <{ [11 x i8] }> <{ [11 x i8] c"src/main.rs" }>, align 1
@alloc25 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [11 x i8] }>, <{ [11 x i8] }>* @alloc24, i32 0, i32 0, i32 0), [16 x i8] c"\0B\00\00\00\00\00\00\00\02\00\00\00\05\00\00\00" }>, align 8
@__rustc_debug_gdb_scripts_section__ = linkonce_odr unnamed_addr constant [34 x i8] c"\01gdb_load_rust_pretty_printers.py\00", section ".debug_gdb_scripts", align 1

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden i64 @_ZN3std2rt10lang_start17h9be6883538d6b66fE(void ()* nonnull %main, i64 %argc, i8** %argv) unnamed_addr #0 !dbg !157 {
start:
  %argv.dbg.spill = alloca i8**, align 8
  %argc.dbg.spill = alloca i64, align 8
  %main.dbg.spill = alloca void ()*, align 8
  %_7 = alloca i8*, align 8
  store void ()* %main, void ()** %main.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata void ()** %main.dbg.spill, metadata !164, metadata !DIExpression()), !dbg !170
  store i64 %argc, i64* %argc.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i64* %argc.dbg.spill, metadata !165, metadata !DIExpression()), !dbg !171
  store i8** %argv, i8*** %argv.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8*** %argv.dbg.spill, metadata !166, metadata !DIExpression()), !dbg !172
  %0 = bitcast i8** %_7 to void ()**, !dbg !173
  store void ()* %main, void ()** %0, align 8, !dbg !173
  %_4.0 = bitcast i8** %_7 to {}*, !dbg !174
; call std::rt::lang_start_internal
  %1 = call i64 @_ZN3std2rt19lang_start_internal17heb6c9d57f3355dedE({}* nonnull align 1 %_4.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) bitcast ({ void (i8**)*, i64, i64, i32 (i8**)*, i32 (i8**)*, i32 (i8**)* }* @vtable.0 to [3 x i64]*), i64 %argc, i8** %argv), !dbg !175
  br label %bb1, !dbg !175

bb1:                                              ; preds = %start
  ret i64 %1, !dbg !176
}

; std::rt::lang_start::{{closure}}
; Function Attrs: nonlazybind uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9037d04980ce8d14E"(i8** noalias readonly align 8 dereferenceable(8) %_1) unnamed_addr #0 !dbg !177 {
start:
  %_1.dbg.spill = alloca i8**, align 8
  store i8** %_1, i8*** %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8*** %_1.dbg.spill, metadata !182, metadata !DIExpression(DW_OP_deref)), !dbg !183
  %0 = bitcast i8** %_1 to void ()**, !dbg !184
  %_3 = load void ()*, void ()** %0, align 8, !dbg !184, !nonnull !4
  call void %_3(), !dbg !184
  br label %bb1, !dbg !184

bb1:                                              ; preds = %start
; call <() as std::process::Termination>::report
  %1 = call i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h108cf09586b254ceE"(), !dbg !184
  br label %bb2, !dbg !184

bb2:                                              ; preds = %bb1
  ret i32 %1, !dbg !185
}

; std::sys::unix::process::process_common::ExitCode::as_i32
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217he845b5a47ea1b3bdE(i8* noalias readonly align 1 dereferenceable(1) %self) unnamed_addr #1 !dbg !186 {
start:
  %self.dbg.spill = alloca i8*, align 8
  store i8* %self, i8** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill, metadata !199, metadata !DIExpression()), !dbg !200
  %_2 = load i8, i8* %self, align 1, !dbg !201
  %0 = zext i8 %_2 to i32, !dbg !201
  ret i32 %0, !dbg !202
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: nonlazybind uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h1aa824039daa9e63E"(i8** %_1) unnamed_addr #0 !dbg !203 {
start:
  %_1.dbg.spill = alloca i8**, align 8
  %_2 = alloca {}, align 1
  store i8** %_1, i8*** %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8*** %_1.dbg.spill, metadata !212, metadata !DIExpression()), !dbg !217
  call void @llvm.dbg.declare(metadata {}* %_2, metadata !213, metadata !DIExpression()), !dbg !217
  %0 = load i8*, i8** %_1, align 8, !dbg !217, !nonnull !4
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h23aee4f3815d4517E(i8* nonnull %0), !dbg !217
  br label %bb1, !dbg !217

bb1:                                              ; preds = %start
  ret i32 %1, !dbg !217
}

; core::ops::function::FnOnce::call_once
; Function Attrs: nonlazybind uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h23aee4f3815d4517E(i8* nonnull %0)) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !218 {
start:
  %1 = alloca { i8*, i32 }, align 8
  %_2 = alloca {}, align 1
  %_1 = alloca i8*, align 8
  store i8* %0, i8** %_1, align 8
  call void @llvm.dbg.declare(metadata i8** %_1, metadata !222, metadata !DIExpression()), !dbg !224
  call void @llvm.dbg.declare(metadata {}* %_2, metadata !223, metadata !DIExpression()), !dbg !224
; invoke std::rt::lang_start::{{closure}}
  %2 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9037d04980ce8d14E"(i8** noalias readonly align 8 dereferenceable(8) %_1)
          to label %bb1 unwind label %cleanup, !dbg !224

bb1:                                              ; preds = %start
  br label %bb2, !dbg !224

bb2:                                              ; preds = %bb1
  ret i32 %2, !dbg !224

bb3:                                              ; preds = %cleanup
  br label %bb4, !dbg !224

bb4:                                              ; preds = %bb3
  %3 = bitcast { i8*, i32 }* %1 to i8**, !dbg !224
  %4 = load i8*, i8** %3, align 8, !dbg !224
  %5 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1, !dbg !224
  %6 = load i32, i32* %5, align 8, !dbg !224
  %7 = insertvalue { i8*, i32 } undef, i8* %4, 0, !dbg !224
  %8 = insertvalue { i8*, i32 } %7, i32 %6, 1, !dbg !224
  resume { i8*, i32 } %8, !dbg !224

cleanup:                                          ; preds = %start
  %9 = landingpad { i8*, i32 }
          cleanup
  %10 = extractvalue { i8*, i32 } %9, 0
  %11 = extractvalue { i8*, i32 } %9, 1
  %12 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %10, i8** %12, align 8
  %13 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %11, i32* %13, align 8
  br label %bb3
}

; core::ptr::drop_in_place
; Function Attrs: nonlazybind uwtable
define internal void @_ZN4core3ptr13drop_in_place17hf761a04b7e1ae360E(i8** %_1) unnamed_addr #0 !dbg !225 {
start:
  %_1.dbg.spill = alloca i8**, align 8
  %0 = alloca {}, align 1
  store i8** %_1, i8*** %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8*** %_1.dbg.spill, metadata !230, metadata !DIExpression()), !dbg !233
  ret void, !dbg !233
}

; core::result::Result<T,E>::unwrap
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h7f0eebccc4ee11d2E"(%"rust_meros::primitives::file::File"* noalias nocapture sret dereferenceable(56) %t, %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>"* noalias nocapture dereferenceable(64) %self, %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) %0) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !234 {
start:
  %1 = alloca { i8*, i32 }, align 8
  %e = alloca %"rust_meros::primitives::file::FileError", align 8
  call void @llvm.dbg.declare(metadata %"rust_meros::primitives::file::File"* %t, metadata !278, metadata !DIExpression()), !dbg !282
  call void @llvm.dbg.declare(metadata %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>"* %self, metadata !277, metadata !DIExpression()), !dbg !283
  call void @llvm.dbg.declare(metadata %"rust_meros::primitives::file::FileError"* %e, metadata !280, metadata !DIExpression()), !dbg !284
  %2 = bitcast %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>"* %self to i64*, !dbg !285
  %_2 = load i64, i64* %2, align 8, !dbg !285, !range !286
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ], !dbg !285

bb1:                                              ; preds = %start
  %3 = bitcast %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>"* %self to %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>::Err"*, !dbg !287
  %4 = getelementptr inbounds %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>::Err", %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>::Err"* %3, i32 0, i32 1, !dbg !287
  %5 = bitcast %"rust_meros::primitives::file::FileError"* %e to i8*, !dbg !287
  %6 = bitcast %"rust_meros::primitives::file::FileError"* %4 to i8*, !dbg !287
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %5, i8* align 8 %6, i64 32, i1 false), !dbg !287
  %_6.0 = bitcast %"rust_meros::primitives::file::FileError"* %e to {}*, !dbg !288
; invoke core::result::unwrap_failed
  invoke void @_ZN4core6result13unwrap_failed17ha34898edcf27887cE([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [43 x i8] }>* @alloc22 to [0 x i8]*), i64 43, {}* nonnull align 1 %_6.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) bitcast ({ void (%"rust_meros::primitives::file::FileError"*)*, i64, i64, i1 (%"rust_meros::primitives::file::FileError"*, %"core::fmt::Formatter"*)* }* @vtable.1 to [3 x i64]*), %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) %0)
          to label %unreachable unwind label %cleanup, !dbg !289

bb2:                                              ; preds = %start
  unreachable, !dbg !290

bb3:                                              ; preds = %start
  %7 = bitcast %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>"* %self to %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>::Ok"*, !dbg !291
  %8 = getelementptr inbounds %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>::Ok", %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>::Ok"* %7, i32 0, i32 1, !dbg !291
  %9 = bitcast %"rust_meros::primitives::file::File"* %t to i8*, !dbg !291
  %10 = bitcast %"rust_meros::primitives::file::File"* %8 to i8*, !dbg !291
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %9, i8* align 8 %10, i64 56, i1 false), !dbg !291
  ret void, !dbg !292

bb4:                                              ; preds = %bb5
  %11 = bitcast { i8*, i32 }* %1 to i8**, !dbg !293
  %12 = load i8*, i8** %11, align 8, !dbg !293
  %13 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1, !dbg !293
  %14 = load i32, i32* %13, align 8, !dbg !293
  %15 = insertvalue { i8*, i32 } undef, i8* %12, 0, !dbg !293
  %16 = insertvalue { i8*, i32 } %15, i32 %14, 1, !dbg !293
  resume { i8*, i32 } %16, !dbg !293

bb5:                                              ; preds = %cleanup
; call core::ptr::drop_in_place
  call void @_ZN4core3ptr13drop_in_place17h509d85da10b4e18bE(%"rust_meros::primitives::file::FileError"* %e) #7, !dbg !294
  br label %bb4, !dbg !294

unreachable:                                      ; preds = %bb1
  unreachable

cleanup:                                          ; preds = %bb1
  %17 = landingpad { i8*, i32 }
          cleanup
  %18 = extractvalue { i8*, i32 } %17, 0
  %19 = extractvalue { i8*, i32 } %17, 1
  %20 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %18, i8** %20, align 8
  %21 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %19, i32* %21, align 8
  br label %bb5
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h108cf09586b254ceE"() unnamed_addr #1 !dbg !295 {
start:
  %self.dbg.spill = alloca {}, align 1
  call void @llvm.dbg.declare(metadata {}* %self.dbg.spill, metadata !302, metadata !DIExpression()), !dbg !303
; call <std::process::ExitCode as std::process::Termination>::report
  %0 = call i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h97e7254ecd856596E"(i8 0), !dbg !304
  br label %bb1, !dbg !304

bb1:                                              ; preds = %start
  ret i32 %0, !dbg !305
}

; <std::process::ExitCode as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h97e7254ecd856596E"(i8 %0) unnamed_addr #1 !dbg !306 {
start:
  %self = alloca i8, align 1
  store i8 %0, i8* %self, align 1
  call void @llvm.dbg.declare(metadata i8* %self, metadata !313, metadata !DIExpression()), !dbg !314
; call std::sys::unix::process::process_common::ExitCode::as_i32
  %1 = call i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217he845b5a47ea1b3bdE(i8* noalias readonly align 1 dereferenceable(1) %self), !dbg !315
  br label %bb1, !dbg !315

bb1:                                              ; preds = %start
  ret i32 %1, !dbg !316
}

; rust_meros::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN10rust_meros4main17h47d34bc1c4ebb1e4E() unnamed_addr #0 !dbg !317 {
start:
  %_2 = alloca %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>", align 8
  %_1 = alloca %"rust_meros::primitives::file::File", align 8
; call std::path::Path::new
  %0 = call { %"std::path::Path"*, i64 } @_ZN3std4path4Path3new17h1d0892bec516958cE([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [10 x i8] }>* @alloc23 to [0 x i8]*), i64 10), !dbg !319
  %_4.0 = extractvalue { %"std::path::Path"*, i64 } %0, 0, !dbg !319
  %_4.1 = extractvalue { %"std::path::Path"*, i64 } %0, 1, !dbg !319
  br label %bb1, !dbg !319

bb1:                                              ; preds = %start
; call rust_meros::primitives::file::File::new
  call void @_ZN10rust_meros10primitives4file4File3new17haa2e16e143606f6fE(%"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>"* noalias nocapture sret dereferenceable(64) %_2, %"std::path::Path"* noalias nonnull readonly align 1 %_4.0, i64 %_4.1), !dbg !320
  br label %bb2, !dbg !320

bb2:                                              ; preds = %bb1
; call core::result::Result<T,E>::unwrap
  call void @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h7f0eebccc4ee11d2E"(%"rust_meros::primitives::file::File"* noalias nocapture sret dereferenceable(56) %_1, %"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>"* noalias nocapture dereferenceable(64) %_2, %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc25 to %"core::panic::Location"*)), !dbg !320
  br label %bb3, !dbg !320

bb3:                                              ; preds = %bb2
; call core::ptr::drop_in_place
  call void @_ZN4core3ptr13drop_in_place17h42e3c93b100d1c68E(%"rust_meros::primitives::file::File"* %_1), !dbg !321
  br label %bb4, !dbg !321

bb4:                                              ; preds = %bb3
  ret void, !dbg !322
}

; Function Attrs: nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #2

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare i64 @_ZN3std2rt19lang_start_internal17heb6c9d57f3355dedE({}* nonnull align 1, [3 x i64]* noalias readonly align 8 dereferenceable(24), i64, i8**) unnamed_addr #0

; Function Attrs: nounwind nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #3

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #4

; core::ptr::drop_in_place
; Function Attrs: nonlazybind uwtable
declare void @_ZN4core3ptr13drop_in_place17h509d85da10b4e18bE(%"rust_meros::primitives::file::FileError"*) unnamed_addr #0

; <rust_meros::primitives::file::FileError as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN76_$LT$rust_meros..primitives..file..FileError$u20$as$u20$core..fmt..Debug$GT$3fmt17h6cd85552ea2d3842E"(%"rust_meros::primitives::file::FileError"* noalias readonly align 8 dereferenceable(32), %"core::fmt::Formatter"* align 8 dereferenceable(64)) unnamed_addr #0

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core6result13unwrap_failed17ha34898edcf27887cE([0 x i8]* noalias nonnull readonly align 1, i64, {}* nonnull align 1, [3 x i64]* noalias readonly align 8 dereferenceable(24), %"core::panic::Location"* noalias readonly align 8 dereferenceable(24)) unnamed_addr #5

; std::path::Path::new
; Function Attrs: nonlazybind uwtable
declare { %"std::path::Path"*, i64 } @_ZN3std4path4Path3new17h1d0892bec516958cE([0 x i8]* noalias nonnull readonly align 1, i64) unnamed_addr #0

; rust_meros::primitives::file::File::new
; Function Attrs: nonlazybind uwtable
declare void @_ZN10rust_meros10primitives4file4File3new17haa2e16e143606f6fE(%"core::result::Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>"* noalias nocapture sret dereferenceable(64), %"std::path::Path"* noalias nonnull readonly align 1, i64) unnamed_addr #0

; core::ptr::drop_in_place
; Function Attrs: nonlazybind uwtable
declare void @_ZN4core3ptr13drop_in_place17h42e3c93b100d1c68E(%"rust_meros::primitives::file::File"*) unnamed_addr #0

; Function Attrs: nonlazybind
define i32 @main(i32 %0, i8** %1) unnamed_addr #6 {
top:
  %2 = load volatile i8, i8* getelementptr inbounds ([34 x i8], [34 x i8]* @__rustc_debug_gdb_scripts_section__, i32 0, i32 0), align 1
  %3 = sext i32 %0 to i64
; call std::rt::lang_start
  %4 = call i64 @_ZN3std2rt10lang_start17h9be6883538d6b66fE(void ()* @_ZN10rust_meros4main17h47d34bc1c4ebb1e4E, i64 %3, i8** %1)
  %5 = trunc i64 %4 to i32
  ret i32 %5
}

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { nounwind readnone speculatable willreturn }
attributes #3 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #4 = { argmemonly nounwind willreturn }
attributes #5 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #6 = { nonlazybind "target-cpu"="x86-64" }
attributes #7 = { noinline }

!llvm.module.flags = !{!149, !150, !151, !152}
!llvm.dbg.cu = !{!153}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "vtable", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "vtable", file: !2, align: 64, flags: DIFlagArtificial, elements: !4, vtableHolder: !5, identifier: "vtable")
!4 = !{}
!5 = !DICompositeType(tag: DW_TAG_structure_type, name: "closure-0", scope: !6, file: !2, size: 64, align: 64, elements: !9, templateParams: !4, identifier: "e5e35ddce6018cb83c6366267769f6a4")
!6 = !DINamespace(name: "lang_start", scope: !7)
!7 = !DINamespace(name: "rt", scope: !8)
!8 = !DINamespace(name: "std", scope: null)
!9 = !{!10}
!10 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !5, file: !2, baseType: !11, size: 64, align: 64)
!11 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn()", baseType: !12, size: 64, align: 64, dwarfAddressSpace: 0)
!12 = !DISubroutineType(types: !13)
!13 = !{null}
!14 = !DIGlobalVariableExpression(var: !15, expr: !DIExpression())
!15 = distinct !DIGlobalVariable(name: "vtable", scope: null, file: !2, type: !16, isLocal: true, isDefinition: true)
!16 = !DICompositeType(tag: DW_TAG_structure_type, name: "vtable", file: !2, align: 64, flags: DIFlagArtificial, elements: !4, vtableHolder: !17, identifier: "vtable")
!17 = !DICompositeType(tag: DW_TAG_structure_type, name: "FileError", scope: !18, file: !2, size: 256, align: 64, elements: !21, identifier: "fa0a45769b1e292dd4044c842a6e69eb")
!18 = !DINamespace(name: "file", scope: !19)
!19 = !DINamespace(name: "primitives", scope: !20)
!20 = !DINamespace(name: "rust_meros", scope: null)
!21 = !{!22}
!22 = !DICompositeType(tag: DW_TAG_variant_part, scope: !18, file: !2, size: 256, align: 64, elements: !23, templateParams: !4, identifier: "fa0a45769b1e292dd4044c842a6e69eb_variant_part", discriminator: !148)
!23 = !{!24, !94, !133}
!24 = !DIDerivedType(tag: DW_TAG_member, name: "IO", scope: !22, file: !2, baseType: !25, size: 256, align: 64, extraData: i64 0)
!25 = !DICompositeType(tag: DW_TAG_structure_type, name: "IO", scope: !17, file: !2, size: 256, align: 64, elements: !26, templateParams: !4, identifier: "fa0a45769b1e292dd4044c842a6e69eb::IO")
!26 = !{!27}
!27 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !25, file: !2, baseType: !28, size: 128, align: 64, offset: 64)
!28 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !29, file: !2, size: 128, align: 64, elements: !31, templateParams: !4, identifier: "2e348066c9d43a6ba5aea30f2b7098b5")
!29 = !DINamespace(name: "error", scope: !30)
!30 = !DINamespace(name: "io", scope: !8)
!31 = !{!32}
!32 = !DIDerivedType(tag: DW_TAG_member, name: "repr", scope: !28, file: !2, baseType: !33, size: 128, align: 64)
!33 = !DICompositeType(tag: DW_TAG_structure_type, name: "Repr", scope: !29, file: !2, size: 128, align: 64, elements: !34, identifier: "742b569a5dc487ea337b0ad5c41ccc65")
!34 = !{!35}
!35 = !DICompositeType(tag: DW_TAG_variant_part, scope: !29, file: !2, size: 128, align: 64, elements: !36, templateParams: !4, identifier: "742b569a5dc487ea337b0ad5c41ccc65_variant_part", discriminator: !93)
!36 = !{!37, !42, !67}
!37 = !DIDerivedType(tag: DW_TAG_member, name: "Os", scope: !35, file: !2, baseType: !38, size: 128, align: 64, extraData: i64 0)
!38 = !DICompositeType(tag: DW_TAG_structure_type, name: "Os", scope: !33, file: !2, size: 128, align: 64, elements: !39, templateParams: !4, identifier: "742b569a5dc487ea337b0ad5c41ccc65::Os")
!39 = !{!40}
!40 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !38, file: !2, baseType: !41, size: 32, align: 32, offset: 32)
!41 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!42 = !DIDerivedType(tag: DW_TAG_member, name: "Simple", scope: !35, file: !2, baseType: !43, size: 128, align: 64, extraData: i64 1)
!43 = !DICompositeType(tag: DW_TAG_structure_type, name: "Simple", scope: !33, file: !2, size: 128, align: 64, elements: !44, templateParams: !4, identifier: "742b569a5dc487ea337b0ad5c41ccc65::Simple")
!44 = !{!45}
!45 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !43, file: !2, baseType: !46, size: 8, align: 8, offset: 8)
!46 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "ErrorKind", scope: !29, file: !2, baseType: !47, size: 8, align: 8, flags: DIFlagEnumClass, elements: !48)
!47 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!48 = !{!49, !50, !51, !52, !53, !54, !55, !56, !57, !58, !59, !60, !61, !62, !63, !64, !65, !66}
!49 = !DIEnumerator(name: "NotFound", value: 0)
!50 = !DIEnumerator(name: "PermissionDenied", value: 1)
!51 = !DIEnumerator(name: "ConnectionRefused", value: 2)
!52 = !DIEnumerator(name: "ConnectionReset", value: 3)
!53 = !DIEnumerator(name: "ConnectionAborted", value: 4)
!54 = !DIEnumerator(name: "NotConnected", value: 5)
!55 = !DIEnumerator(name: "AddrInUse", value: 6)
!56 = !DIEnumerator(name: "AddrNotAvailable", value: 7)
!57 = !DIEnumerator(name: "BrokenPipe", value: 8)
!58 = !DIEnumerator(name: "AlreadyExists", value: 9)
!59 = !DIEnumerator(name: "WouldBlock", value: 10)
!60 = !DIEnumerator(name: "InvalidInput", value: 11)
!61 = !DIEnumerator(name: "InvalidData", value: 12)
!62 = !DIEnumerator(name: "TimedOut", value: 13)
!63 = !DIEnumerator(name: "WriteZero", value: 14)
!64 = !DIEnumerator(name: "Interrupted", value: 15)
!65 = !DIEnumerator(name: "Other", value: 16)
!66 = !DIEnumerator(name: "UnexpectedEof", value: 17)
!67 = !DIDerivedType(tag: DW_TAG_member, name: "Custom", scope: !35, file: !2, baseType: !68, size: 128, align: 64, extraData: i64 2)
!68 = !DICompositeType(tag: DW_TAG_structure_type, name: "Custom", scope: !33, file: !2, size: 128, align: 64, elements: !69, templateParams: !4, identifier: "742b569a5dc487ea337b0ad5c41ccc65::Custom")
!69 = !{!70}
!70 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !68, file: !2, baseType: !71, size: 64, align: 64, offset: 64)
!71 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "Box<std::io::error::Custom>", baseType: !72, size: 64, align: 64, dwarfAddressSpace: 0)
!72 = !DICompositeType(tag: DW_TAG_structure_type, name: "Custom", scope: !29, file: !2, size: 192, align: 64, elements: !73, templateParams: !4, identifier: "6695dc01ec56375ae7018945460a22c5")
!73 = !{!74, !75}
!74 = !DIDerivedType(tag: DW_TAG_member, name: "kind", scope: !72, file: !2, baseType: !46, size: 8, align: 8, offset: 128)
!75 = !DIDerivedType(tag: DW_TAG_member, name: "error", scope: !72, file: !2, baseType: !76, size: 128, align: 64)
!76 = !DICompositeType(tag: DW_TAG_structure_type, name: "Box<Error>", scope: !77, file: !2, size: 128, align: 64, elements: !78, templateParams: !87, identifier: "870ef0df03c39aedab838da24eb78c20")
!77 = !DINamespace(name: "error", scope: !8)
!78 = !{!79, !81}
!79 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !76, file: !2, baseType: !80, size: 64, align: 64, flags: DIFlagArtificial)
!80 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u8", baseType: !47, size: 64, align: 64, dwarfAddressSpace: 0)
!81 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !76, file: !2, baseType: !82, size: 64, align: 64, offset: 64, flags: DIFlagArtificial)
!82 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 3]", baseType: !83, size: 64, align: 64, dwarfAddressSpace: 0)
!83 = !DICompositeType(tag: DW_TAG_array_type, baseType: !84, size: 192, align: 64, elements: !85)
!84 = !DIBasicType(name: "usize", size: 64, encoding: DW_ATE_unsigned)
!85 = !{!86}
!86 = !DISubrange(count: 3)
!87 = !{!88}
!88 = !DITemplateTypeParameter(name: "T", type: !89)
!89 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !77, file: !2, align: 8, elements: !90, templateParams: !4, identifier: "2ea3f799b81d93f7f691f01a660be67")
!90 = !{!91, !92}
!91 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !89, file: !2, baseType: !80, size: 64, align: 64, flags: DIFlagArtificial)
!92 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !89, file: !2, baseType: !82, size: 64, align: 64, offset: 64, flags: DIFlagArtificial)
!93 = !DIDerivedType(tag: DW_TAG_member, scope: !29, file: !2, baseType: !47, size: 8, align: 8, flags: DIFlagArtificial)
!94 = !DIDerivedType(tag: DW_TAG_member, name: "InvalidFilepath", scope: !22, file: !2, baseType: !95, size: 256, align: 64, extraData: i64 1)
!95 = !DICompositeType(tag: DW_TAG_structure_type, name: "InvalidFilepath", scope: !17, file: !2, size: 256, align: 64, elements: !96, templateParams: !4, identifier: "fa0a45769b1e292dd4044c842a6e69eb::InvalidFilepath")
!96 = !{!97}
!97 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !95, file: !2, baseType: !98, size: 192, align: 64, offset: 64)
!98 = !DICompositeType(tag: DW_TAG_structure_type, name: "GeneralError", scope: !20, file: !2, size: 192, align: 64, elements: !99, templateParams: !4, identifier: "dd97bf32ade2daa2bd6dbac368a8f4ba")
!99 = !{!100}
!100 = !DIDerivedType(tag: DW_TAG_member, name: "details", scope: !98, file: !2, baseType: !101, size: 192, align: 64)
!101 = !DICompositeType(tag: DW_TAG_structure_type, name: "String", scope: !102, file: !2, size: 192, align: 64, elements: !104, templateParams: !4, identifier: "6e6f89c521584c99c23a4006480709f2")
!102 = !DINamespace(name: "string", scope: !103)
!103 = !DINamespace(name: "alloc", scope: null)
!104 = !{!105}
!105 = !DIDerivedType(tag: DW_TAG_member, name: "vec", scope: !101, file: !2, baseType: !106, size: 192, align: 64)
!106 = !DICompositeType(tag: DW_TAG_structure_type, name: "Vec<u8>", scope: !107, file: !2, size: 192, align: 64, elements: !108, templateParams: !124, identifier: "141f1eb741c6d9242f67c7354d668f78")
!107 = !DINamespace(name: "vec", scope: !103)
!108 = !{!109, !132}
!109 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !106, file: !2, baseType: !110, size: 128, align: 64)
!110 = !DICompositeType(tag: DW_TAG_structure_type, name: "RawVec<u8, alloc::alloc::Global>", scope: !111, file: !2, size: 128, align: 64, elements: !112, templateParams: !130, identifier: "4ffc10e59d95dcf826f03751ed6d1039")
!111 = !DINamespace(name: "raw_vec", scope: !103)
!112 = !{!113, !126, !127}
!113 = !DIDerivedType(tag: DW_TAG_member, name: "ptr", scope: !110, file: !2, baseType: !114, size: 64, align: 64)
!114 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unique<u8>", scope: !115, file: !2, size: 64, align: 64, elements: !118, templateParams: !124, identifier: "ab0a0df8080d22413342ef60939111ce")
!115 = !DINamespace(name: "unique", scope: !116)
!116 = !DINamespace(name: "ptr", scope: !117)
!117 = !DINamespace(name: "core", scope: null)
!118 = !{!119, !121}
!119 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !114, file: !2, baseType: !120, size: 64, align: 64)
!120 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !47, size: 64, align: 64, dwarfAddressSpace: 0)
!121 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !114, file: !2, baseType: !122, align: 8)
!122 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<u8>", scope: !123, file: !2, align: 8, elements: !4, templateParams: !124, identifier: "95b022adf1a88718764fd5567fef8541")
!123 = !DINamespace(name: "marker", scope: !117)
!124 = !{!125}
!125 = !DITemplateTypeParameter(name: "T", type: !47)
!126 = !DIDerivedType(tag: DW_TAG_member, name: "cap", scope: !110, file: !2, baseType: !84, size: 64, align: 64, offset: 64)
!127 = !DIDerivedType(tag: DW_TAG_member, name: "alloc", scope: !110, file: !2, baseType: !128, align: 8)
!128 = !DICompositeType(tag: DW_TAG_structure_type, name: "Global", scope: !129, file: !2, align: 8, elements: !4, templateParams: !4, identifier: "a9bef5bedb75939b63f7b42efe9e40c6")
!129 = !DINamespace(name: "alloc", scope: !103)
!130 = !{!125, !131}
!131 = !DITemplateTypeParameter(name: "A", type: !128)
!132 = !DIDerivedType(tag: DW_TAG_member, name: "len", scope: !106, file: !2, baseType: !84, size: 64, align: 64, offset: 128)
!133 = !DIDerivedType(tag: DW_TAG_member, name: "SystemTimeError", scope: !22, file: !2, baseType: !134, size: 256, align: 64, extraData: i64 2)
!134 = !DICompositeType(tag: DW_TAG_structure_type, name: "SystemTimeError", scope: !17, file: !2, size: 256, align: 64, elements: !135, templateParams: !4, identifier: "fa0a45769b1e292dd4044c842a6e69eb::SystemTimeError")
!135 = !{!136}
!136 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !134, file: !2, baseType: !137, size: 128, align: 64, offset: 64)
!137 = !DICompositeType(tag: DW_TAG_structure_type, name: "SystemTimeError", scope: !138, file: !2, size: 128, align: 64, elements: !139, templateParams: !4, identifier: "e6a6d6693dd3deb0f9f9dcea6a2e7d9e")
!138 = !DINamespace(name: "time", scope: !8)
!139 = !{!140}
!140 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !137, file: !2, baseType: !141, size: 128, align: 64)
!141 = !DICompositeType(tag: DW_TAG_structure_type, name: "Duration", scope: !142, file: !2, size: 128, align: 64, elements: !143, templateParams: !4, identifier: "fd5db0e5449cad66b22deb016851581")
!142 = !DINamespace(name: "time", scope: !117)
!143 = !{!144, !146}
!144 = !DIDerivedType(tag: DW_TAG_member, name: "secs", scope: !141, file: !2, baseType: !145, size: 64, align: 64)
!145 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!146 = !DIDerivedType(tag: DW_TAG_member, name: "nanos", scope: !141, file: !2, baseType: !147, size: 32, align: 32, offset: 64)
!147 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!148 = !DIDerivedType(tag: DW_TAG_member, scope: !18, file: !2, baseType: !145, size: 64, align: 64, flags: DIFlagArtificial)
!149 = !{i32 7, !"PIC Level", i32 2}
!150 = !{i32 7, !"PIE Level", i32 2}
!151 = !{i32 2, !"RtLibUseGOT", i32 1}
!152 = !{i32 2, !"Debug Info Version", i32 3}
!153 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !154, producer: "clang LLVM (rustc version 1.45.1 (c367798cf 2020-07-26))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !155, globals: !156)
!154 = !DIFile(filename: "src/main.rs", directory: "/home/matt/rust/rust-meros")
!155 = !{!46}
!156 = !{!0, !14}
!157 = distinct !DISubprogram(name: "lang_start<()>", linkageName: "_ZN3std2rt10lang_start17h9be6883538d6b66fE", scope: !7, file: !158, line: 62, type: !159, scopeLine: 62, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !153, templateParams: !167, retainedNodes: !163)
!158 = !DIFile(filename: "/rustc/c367798cfd3817ca6ae908ce675d1d99242af148/src/libstd/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "580128330782ec7d1bbdaeb3149adfa0")
!159 = !DISubroutineType(types: !160)
!160 = !{!161, !11, !161, !162}
!161 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!162 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !120, size: 64, align: 64, dwarfAddressSpace: 0)
!163 = !{!164, !165, !166}
!164 = !DILocalVariable(name: "main", arg: 1, scope: !157, file: !158, line: 63, type: !11)
!165 = !DILocalVariable(name: "argc", arg: 2, scope: !157, file: !158, line: 64, type: !161)
!166 = !DILocalVariable(name: "argv", arg: 3, scope: !157, file: !158, line: 65, type: !162)
!167 = !{!168}
!168 = !DITemplateTypeParameter(name: "T", type: !169)
!169 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!170 = !DILocation(line: 63, column: 5, scope: !157)
!171 = !DILocation(line: 64, column: 5, scope: !157)
!172 = !DILocation(line: 65, column: 5, scope: !157)
!173 = !DILocation(line: 67, column: 26, scope: !157)
!174 = !DILocation(line: 67, column: 25, scope: !157)
!175 = !DILocation(line: 67, column: 5, scope: !157)
!176 = !DILocation(line: 68, column: 2, scope: !157)
!177 = distinct !DISubprogram(name: "{{closure}}<()>", linkageName: "_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9037d04980ce8d14E", scope: !6, file: !158, line: 67, type: !178, scopeLine: 67, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !153, templateParams: !167, retainedNodes: !181)
!178 = !DISubroutineType(types: !179)
!179 = !{!41, !180}
!180 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&closure-0", baseType: !5, size: 64, align: 64, dwarfAddressSpace: 0)
!181 = !{!182}
!182 = !DILocalVariable(name: "main", scope: !177, file: !158, line: 63, type: !11, align: 8)
!183 = !DILocation(line: 63, column: 5, scope: !177)
!184 = !DILocation(line: 67, column: 34, scope: !177)
!185 = !DILocation(line: 67, column: 49, scope: !177)
!186 = distinct !DISubprogram(name: "as_i32", linkageName: "_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217he845b5a47ea1b3bdE", scope: !188, file: !187, line: 398, type: !195, scopeLine: 398, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !153, templateParams: !4, retainedNodes: !198)
!187 = !DIFile(filename: "/rustc/c367798cfd3817ca6ae908ce675d1d99242af148/src/libstd/sys/unix/process/process_common.rs", directory: "", checksumkind: CSK_MD5, checksum: "620f5402e8ba314453e210d5cb93d802")
!188 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !189, file: !2, size: 8, align: 8, elements: !193, templateParams: !4, identifier: "500b55529ad4719cf652d72b02845feb")
!189 = !DINamespace(name: "process_common", scope: !190)
!190 = !DINamespace(name: "process", scope: !191)
!191 = !DINamespace(name: "unix", scope: !192)
!192 = !DINamespace(name: "sys", scope: !8)
!193 = !{!194}
!194 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !188, file: !2, baseType: !47, size: 8, align: 8)
!195 = !DISubroutineType(types: !196)
!196 = !{!41, !197}
!197 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::sys::unix::process::process_common::ExitCode", baseType: !188, size: 64, align: 64, dwarfAddressSpace: 0)
!198 = !{!199}
!199 = !DILocalVariable(name: "self", arg: 1, scope: !186, file: !187, line: 398, type: !197)
!200 = !DILocation(line: 398, column: 19, scope: !186)
!201 = !DILocation(line: 399, column: 9, scope: !186)
!202 = !DILocation(line: 400, column: 6, scope: !186)
!203 = distinct !DISubprogram(name: "call_once<closure-0,()>", linkageName: "_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h1aa824039daa9e63E", scope: !205, file: !204, line: 232, type: !208, scopeLine: 232, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !153, templateParams: !214, retainedNodes: !211)
!204 = !DIFile(filename: "/rustc/c367798cfd3817ca6ae908ce675d1d99242af148/src/libcore/ops/function.rs", directory: "", checksumkind: CSK_MD5, checksum: "4e38de05081e36b7316ccfac06754856")
!205 = !DINamespace(name: "FnOnce", scope: !206)
!206 = !DINamespace(name: "function", scope: !207)
!207 = !DINamespace(name: "ops", scope: !117)
!208 = !DISubroutineType(types: !209)
!209 = !{!41, !210}
!210 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut closure-0", baseType: !5, size: 64, align: 64, dwarfAddressSpace: 0)
!211 = !{!212, !213}
!212 = !DILocalVariable(arg: 1, scope: !203, file: !204, line: 232, type: !210)
!213 = !DILocalVariable(arg: 2, scope: !203, file: !204, line: 232, type: !169)
!214 = !{!215, !216}
!215 = !DITemplateTypeParameter(name: "Self", type: !5)
!216 = !DITemplateTypeParameter(name: "Args", type: !169)
!217 = !DILocation(line: 232, column: 5, scope: !203)
!218 = distinct !DISubprogram(name: "call_once<closure-0,()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17h23aee4f3815d4517E", scope: !205, file: !204, line: 232, type: !219, scopeLine: 232, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !153, templateParams: !214, retainedNodes: !221)
!219 = !DISubroutineType(types: !220)
!220 = !{!41, !5}
!221 = !{!222, !223}
!222 = !DILocalVariable(arg: 1, scope: !218, file: !204, line: 232, type: !5)
!223 = !DILocalVariable(arg: 2, scope: !218, file: !204, line: 232, type: !169)
!224 = !DILocation(line: 232, column: 5, scope: !218)
!225 = distinct !DISubprogram(name: "drop_in_place<closure-0>", linkageName: "_ZN4core3ptr13drop_in_place17hf761a04b7e1ae360E", scope: !116, file: !226, line: 184, type: !227, scopeLine: 184, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !153, templateParams: !231, retainedNodes: !229)
!226 = !DIFile(filename: "/rustc/c367798cfd3817ca6ae908ce675d1d99242af148/src/libcore/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "3a5ba522fe2039fe5996ec730d3af763")
!227 = !DISubroutineType(types: !228)
!228 = !{null, !210}
!229 = !{!230}
!230 = !DILocalVariable(arg: 1, scope: !225, file: !226, line: 184, type: !210)
!231 = !{!232}
!232 = !DITemplateTypeParameter(name: "T", type: !5)
!233 = !DILocation(line: 184, column: 1, scope: !225)
!234 = distinct !DISubprogram(name: "unwrap<rust_meros::primitives::file::File,rust_meros::primitives::file::FileError>", linkageName: "_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h7f0eebccc4ee11d2E", scope: !236, file: !235, line: 1002, type: !263, scopeLine: 1002, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !153, templateParams: !255, retainedNodes: !276)
!235 = !DIFile(filename: "/rustc/c367798cfd3817ca6ae908ce675d1d99242af148/src/libcore/result.rs", directory: "", checksumkind: CSK_MD5, checksum: "6bdeed70a3678de0dd25256a0a378602")
!236 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<rust_meros::primitives::file::File, rust_meros::primitives::file::FileError>", scope: !237, file: !2, size: 512, align: 64, elements: !238, identifier: "ab9fa40dc0c06477668b383c079c20fc")
!237 = !DINamespace(name: "result", scope: !117)
!238 = !{!239}
!239 = !DICompositeType(tag: DW_TAG_variant_part, scope: !237, file: !2, size: 512, align: 64, elements: !240, templateParams: !255, identifier: "ab9fa40dc0c06477668b383c079c20fc_variant_part", discriminator: !262)
!240 = !{!241, !258}
!241 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !239, file: !2, baseType: !242, size: 512, align: 64, extraData: i64 0)
!242 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !236, file: !2, size: 512, align: 64, elements: !243, templateParams: !255, identifier: "ab9fa40dc0c06477668b383c079c20fc::Ok")
!243 = !{!244}
!244 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !242, file: !2, baseType: !245, size: 448, align: 64, offset: 64)
!245 = !DICompositeType(tag: DW_TAG_structure_type, name: "File", scope: !18, file: !2, size: 448, align: 64, elements: !246, templateParams: !4, identifier: "42b5513870943c70b6db49322a677335")
!246 = !{!247, !248}
!247 = !DIDerivedType(tag: DW_TAG_member, name: "filename", scope: !245, file: !2, baseType: !101, size: 192, align: 64)
!248 = !DIDerivedType(tag: DW_TAG_member, name: "id", scope: !245, file: !2, baseType: !249, size: 256, align: 8, offset: 192)
!249 = !DICompositeType(tag: DW_TAG_structure_type, name: "FileID", scope: !18, file: !2, size: 256, align: 8, elements: !250, templateParams: !4, identifier: "13b689c8af7911aa43c38fac52d48217")
!250 = !{!251}
!251 = !DIDerivedType(tag: DW_TAG_member, name: "id", scope: !249, file: !2, baseType: !252, size: 256, align: 8)
!252 = !DICompositeType(tag: DW_TAG_array_type, baseType: !47, size: 256, align: 8, elements: !253)
!253 = !{!254}
!254 = !DISubrange(count: 32)
!255 = !{!256, !257}
!256 = !DITemplateTypeParameter(name: "T", type: !245)
!257 = !DITemplateTypeParameter(name: "E", type: !17)
!258 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !239, file: !2, baseType: !259, size: 512, align: 64, extraData: i64 1)
!259 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !236, file: !2, size: 512, align: 64, elements: !260, templateParams: !255, identifier: "ab9fa40dc0c06477668b383c079c20fc::Err")
!260 = !{!261}
!261 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !259, file: !2, baseType: !17, size: 256, align: 64, offset: 64)
!262 = !DIDerivedType(tag: DW_TAG_member, scope: !237, file: !2, baseType: !145, size: 64, align: 64, flags: DIFlagArtificial)
!263 = !DISubroutineType(types: !264)
!264 = !{!245, !236, !265}
!265 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::panic::Location", baseType: !266, size: 64, align: 64, dwarfAddressSpace: 0)
!266 = !DICompositeType(tag: DW_TAG_structure_type, name: "Location", scope: !267, file: !2, size: 192, align: 64, elements: !268, templateParams: !4, identifier: "dca2331cc074acc3de9f74cd87f32c8c")
!267 = !DINamespace(name: "panic", scope: !117)
!268 = !{!269, !274, !275}
!269 = !DIDerivedType(tag: DW_TAG_member, name: "file", scope: !266, file: !2, baseType: !270, size: 128, align: 64)
!270 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 128, align: 64, elements: !271, templateParams: !4, identifier: "7ef2a91eecc7bcf4b4aaea2dbce79437")
!271 = !{!272, !273}
!272 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !270, file: !2, baseType: !120, size: 64, align: 64)
!273 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !270, file: !2, baseType: !84, size: 64, align: 64, offset: 64)
!274 = !DIDerivedType(tag: DW_TAG_member, name: "line", scope: !266, file: !2, baseType: !147, size: 32, align: 32, offset: 128)
!275 = !DIDerivedType(tag: DW_TAG_member, name: "col", scope: !266, file: !2, baseType: !147, size: 32, align: 32, offset: 160)
!276 = !{!277, !278, !280}
!277 = !DILocalVariable(name: "self", arg: 1, scope: !234, file: !235, line: 1002, type: !236)
!278 = !DILocalVariable(name: "t", scope: !279, file: !235, line: 1004, type: !245, align: 8)
!279 = distinct !DILexicalBlock(scope: !234, file: !235, line: 1004, column: 13)
!280 = !DILocalVariable(name: "e", scope: !281, file: !235, line: 1005, type: !17, align: 8)
!281 = distinct !DILexicalBlock(scope: !234, file: !235, line: 1005, column: 13)
!282 = !DILocation(line: 1004, column: 16, scope: !279)
!283 = !DILocation(line: 1002, column: 19, scope: !234)
!284 = !DILocation(line: 1005, column: 17, scope: !281)
!285 = !DILocation(line: 1004, column: 13, scope: !234)
!286 = !{i64 0, i64 2}
!287 = !DILocation(line: 1005, column: 17, scope: !234)
!288 = !DILocation(line: 1005, column: 84, scope: !281)
!289 = !DILocation(line: 1005, column: 23, scope: !281)
!290 = !DILocation(line: 1003, column: 15, scope: !234)
!291 = !DILocation(line: 1004, column: 16, scope: !234)
!292 = !DILocation(line: 1007, column: 6, scope: !234)
!293 = !DILocation(line: 1002, column: 5, scope: !234)
!294 = !DILocation(line: 1005, column: 87, scope: !234)
!295 = distinct !DISubprogram(name: "report", linkageName: "_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h108cf09586b254ceE", scope: !297, file: !296, line: 1667, type: !299, scopeLine: 1667, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !153, templateParams: !4, retainedNodes: !301)
!296 = !DIFile(filename: "/rustc/c367798cfd3817ca6ae908ce675d1d99242af148/src/libstd/process.rs", directory: "", checksumkind: CSK_MD5, checksum: "bfa554ca649d4c8c8d446b86ff55b4bf")
!297 = !DINamespace(name: "{{impl}}", scope: !298)
!298 = !DINamespace(name: "process", scope: !8)
!299 = !DISubroutineType(types: !300)
!300 = !{!41, !169}
!301 = !{!302}
!302 = !DILocalVariable(name: "self", arg: 1, scope: !295, file: !296, line: 1667, type: !169)
!303 = !DILocation(line: 1667, column: 15, scope: !295)
!304 = !DILocation(line: 1668, column: 9, scope: !295)
!305 = !DILocation(line: 1669, column: 6, scope: !295)
!306 = distinct !DISubprogram(name: "report", linkageName: "_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h97e7254ecd856596E", scope: !297, file: !296, line: 1701, type: !307, scopeLine: 1701, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !153, templateParams: !4, retainedNodes: !312)
!307 = !DISubroutineType(types: !308)
!308 = !{!41, !309}
!309 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !298, file: !2, size: 8, align: 8, elements: !310, templateParams: !4, identifier: "951bf86a43cda681df98bc1c7b590ef6")
!310 = !{!311}
!311 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !309, file: !2, baseType: !188, size: 8, align: 8)
!312 = !{!313}
!313 = !DILocalVariable(name: "self", arg: 1, scope: !306, file: !296, line: 1701, type: !309)
!314 = !DILocation(line: 1701, column: 15, scope: !306)
!315 = !DILocation(line: 1702, column: 9, scope: !306)
!316 = !DILocation(line: 1703, column: 6, scope: !306)
!317 = distinct !DISubprogram(name: "main", linkageName: "_ZN10rust_meros4main17h47d34bc1c4ebb1e4E", scope: !20, file: !318, line: 1, type: !12, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagMainSubprogram, unit: !153, templateParams: !4, retainedNodes: !4)
!318 = !DIFile(filename: "src/main.rs", directory: "/home/matt/rust/rust-meros", checksumkind: CSK_MD5, checksum: "65387fe69d8b0b7c183bf0c40020cbd6")
!319 = !DILocation(line: 2, column: 45, scope: !317)
!320 = !DILocation(line: 2, column: 5, scope: !317)
!321 = !DILocation(line: 5, column: 14, scope: !317)
!322 = !DILocation(line: 6, column: 2, scope: !317)
