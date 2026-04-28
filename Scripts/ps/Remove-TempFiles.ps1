param ( 
	[switch]$yes
)

class DataSize {
	[int64]$Bytes

	DataSize([int64]$bytes) {
		$this.Bytes = $bytes
	}
}
Update-TypeData -TypeName "DataSize" -MemberType ScriptProperty -MemberName KB -Value { $this.Bytes / 1KB } -Force
Update-TypeData -TypeName "DataSize" -MemberType ScriptProperty -MemberName MB -Value { $this.Bytes / 1MB } -Force
Update-TypeData -TypeName "DataSize" -MemberType ScriptProperty -MemberName GB -Value { $this.Bytes / 1GB } -Force
Update-TypeData -TypeName "DataSize" -MemberType ScriptProperty -MemberName TB -Value { $this.Bytes / 1TB } -Force
Update-TypeData -TypeName "DataSize" -MemberType ScriptProperty -MemberName PB -Value { $this.Bytes / 1PB } -Force


function Get-FolderSize {
    param (
        [string]$Path
    )
	$totalSize = [DataSize]::new(0)
	$totalSize.Bytes = (Get-ChildItem -Path $Path -Recurse -File -ErrorAction SilentlyContinue |
              Measure-Object -Property Length -Sum).Sum -as [int64]

    return ($totalSize.Bytes)
}


function Remove-Files {
	param (
		[array]$files
	)

	$confirm = if ($yes) { "N" } else {Read-Host "Require confimation? (Y/N)"}

	$freedSpace = [DataSize]::new(0)

	$tempFolder = (Get-Item $env:TEMP).FullName.TrimEnd('\')

	foreach ($file in $files) {
		if ($confirm.ToUpper() -ne "N") {
			Write-Host "Processing file: $($file.FullName)"
			Write-Host "Temp folder: $tempFolder"
			$deleteConfirm = Read-Host "Delete this file? (Y/N)"
			if ($deleteConfirm.ToUpper() -ne "Y") {
				Write-Host "Skipping file: $($file.FullName)"
				continue
			}	
		}

		if ($null -eq $file.FullName) {
			Write-Host "Skipping file with null path."
			continue
		}	

		if (-not ($file.DirectoryName.StartsWith($tempFolder, [System.StringComparison]::OrdinalIgnoreCase))) {
			Write-Host "Skipping file outside of temp directory: $($file.FullName)"
			continue
		}

		if ($file.Attributes -band [System.IO.FileAttributes]::ReadOnly) {
			Write-Host "Skipping read-only file: $($file.Name)"
			continue
		}

		try {
			Remove-Item -Path $file.FullName -Force -ErrorAction Stop
			Write-Host "Deleted: $($file.FullName)"
			$freedSpace.Bytes += $file.Length
		} catch [System.IO.IOException] {
			Write-Host "Skipping file in use: $($file.FullName)"
			continue
		} catch [System.ArgumentException] {
			Write-Host "Skipping file with invalid path: $($file.FullName)"
			continue
		}
		catch {
			Write-Host "Failed to delete: $($file.FullName). Error: $_"
			continue
		}
	}
	Write-Host "Total space freed:`n$($freedSpace.Bytes) bytes, $($freedSpace.KB) KB, $($freedSpace.MB) MB"
}


function Test-Paths {
	param (
		[array]$files
	)
	$file = $files[4]
	Write-Host "FullName: $($file.FullName) `nPath: $($file.DirectoryName) `nName: $($file.Name) `nDirectoryName: $($file.DirectoryName) `nBaseName: $($file.BaseName) `nExtension: $($file.Extension)"
	Write-Host "------------------------------------------"

}

#==================================================#
# - - - - - - -  Here  begins  main  - - - - - - - #
#==================================================#



$files = Get-ChildItem -Path "$env:TEMP" -Recurse -File -ErrorAction SilentlyContinue

$totalSize = [DataSize]::new(0)
$totalSize.Bytes = Get-FolderSize -Path "$env:TEMP"


if ($files.Count -eq 0) {
	Write-Host "No files found in the temp directory."
	exit
}

if ($yes) {
	Write-Host "Auto-confirm enabled. Deleting files without confirmation."
	Remove-Files -files $files
	Write-Host "All files in the temp directory have been processed."
	exit
}

$randomFile = $files | Get-Random

Write-Host "Found $($files.Count) files in the temp directory, taking up $($totalSize.Bytes) bytes.`nA random file is: $($randomFile).`nWould you like to delete them? (Y/N)"
$response = Read-Host

$loop = 1
while ($loop -gt 0) {
	switch ($response.ToUpper()) {
		"Y" {
			Remove-Files -files $files
			Write-Host "All files in the temp directory have been processed."
			$loop = 0
		}
		"N" {
			Write-Host "Files not deleted."
			$loop = 0
		}

		"Å" {
			Write-Host "Testing paths..."
			Test-Paths -files $files
			$loop = 0
		}
		default {
			Write-Host "Invalid response. Please enter 'Y' or 'N'."
		}
	}
}

