# Rust WinRT Powershell Sample

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

## This is based on the following samples here
1. https://github.com/microsoft/windows-rs/blob/master/crates/samples/components/json_validator_winrt/src/sample.idl
1. https://github.com/MicrosoftDocs/powershell-sdk-samples/blob/main/SDK-3.0/Get-Process%20Sample%2002/C%23/GetProcessSample02.cs
