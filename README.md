# Rust WinRT Powershell Sample
## Table of Contents
1. [Build Requirements](#build-requirements)
1. [Sample Design](#sample-design)
1. [How to build](#how-to-build)
1. [How to use (Import-Module)](#how-to-use-import-module)
1. [Unload?](#unload)
1. [cmdlet](#cmdlet)
    1. [`Get-BasicTest`](#get-basictest)
    1. [`Get-BasicTestDatabase`](#get-basictestdatabase)
1. [This is based on the following samples here](#this-is-based-on-the-following-samples-here)

## Build Requirements

1. Rust tools
1. Visual Studio 2022 for building the WinRT bindings
    1. Plus some C++ stuff
1. Powershell 7
    1. Older powershell fails with `import-module : Could not load file or assembly 'System.Runtime, Version=9.0.0.0, Culture=neutral, PublicKeyToken=b03f5f7f11d50a3a' or one of its dependencies. The system cannot find the file specified.`
1. .Net 9.0 SDK?

    https://dotnet.microsoft.com/en-us/download/dotnet/thank-you/sdk-9.0.301-windows-x64-installer

1. Powershell SDK?

    `dotnet add package Microsoft.PowerShell.SDK --version 7.5.1`


## Sample Design 

This sample is designed with the following structure:
### `rs_winrt_sample.dll`
The DLL made from Rust that implements the WinRT API defined in `rs_winrt_sample.idl`
1. `pure_rust_types.rs`
    1. This implements the objects with normal Rust structs and traits.
1. `bindings.rs`
    1. This contains the generated WinRT structs and traits, this should not be modified by the developer.
1. `winrt_rust_types.rs`
    1. This contains the developer's implementation of the WinRT structs and traits.
    1. This is split from the pure implementation because these objects may be hard to implement cleanly
    1. This is much harder to debug because of the generated code.
1. `lib.rs`
    1. Sets up the globals to make this DLL publicly export the WinRT types
### `rs_winrt_sample_cs_ps.dll`
The DLL that implements the C# PowerShell module
1. `GetBasicTestCommand.cs`
    1. Implements both Powershell cmdlets

## How to build

1. Open Developer Powershell for VS 2022
1. `cargo build`
1. Build rs_winrt_sample_ps.sln in Visual Studio or using `dotnet`

## How to use (Import-Module) 

You need to copy `rs_winrt_sample.dll` and `WinRT.Runtime.dll` to the same directory as `rs_winrt_sample_cs_ps.dll`

Load this using
`Import-Module ./rs_winrt_sample_cs_ps.dll` or `ipmo ./rs_winrt_sample_cs_ps.dll`.

### Errors:
```
>ipmo rs_winrt_sample_cs_ps.dll
Import-Module: The specified module 'rs_winrt_sample_cs_ps.dll' was not loaded because no valid module file was found in any module directory.
```
should be `ipmo ./rs_winrt_sample_cs_ps.dll`

## Unload?

I have found that the most consistent way to unload is close the powershell process.

## cmdlet

### `Get-BasicTest`

A simple cmdlet to get an instance of the WinRT class implemented in Rust.

You can read `PropertyTest`:
```pwsh
$basic = Get-BasicTest
$basic.PropertyTest
```
Or set it:
```pwsh
$basic.PropertyTest = "HELLO WORLD"
```

You can call `Run` 
```pwsh
$basic.Run("HELLO")
```

### `Get-BasicTestDatabase`

```pwsh
$values = Get-BasicTestDatabase -Number 10
```


## This is based on the following samples here
1. https://github.com/microsoft/windows-rs/blob/master/crates/samples/components/json_validator_winrt/src/sample.idl
1. https://github.com/MicrosoftDocs/powershell-sdk-samples/blob/main/SDK-3.0/Get-Process%20Sample%2002/C%23/GetProcessSample02.cs
