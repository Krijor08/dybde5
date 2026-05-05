param (
	[switch]$verbose,
	[switch]$yes
)

class DataSize {
	[int64]$Bytes

	DataSize([int64]$bytes) {
		$this.Bytes = [math]::Round($bytes, 0, [System.MidpointRounding]::AwayFromZero)
	}
}
Update-TypeData -TypeName "DataSize" -MemberType ScriptProperty -MemberName KB -Value { [math]::Round($this.Bytes / 1KB, 2, [System.MidpointRounding]::AwayFromZero) } -Force
Update-TypeData -TypeName "DataSize" -MemberType ScriptProperty -MemberName MB -Value { [math]::Round($this.Bytes / 1MB, 2, [System.MidpointRounding]::AwayFromZero) } -Force
Update-TypeData -TypeName "DataSize" -MemberType ScriptProperty -MemberName GB -Value { [math]::Round($this.Bytes / 1GB, 2, [System.MidpointRounding]::AwayFromZero) } -Force
Update-TypeData -TypeName "DataSize" -MemberType ScriptProperty -MemberName TB -Value { [math]::Round($this.Bytes / 1TB, 2, [System.MidpointRounding]::AwayFromZero) } -Force
Update-TypeData -TypeName "DataSize" -MemberType ScriptProperty -MemberName PB -Value { [math]::Round($this.Bytes / 1PB, 2, [System.MidpointRounding]::AwayFromZero) } -Force


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

	
	$confirm =	if ($yes) { "N" } else { Read-Host "Require confimation for each file? (Y/N)" }

	$freedSpace = [DataSize]::new(0)

	$tempFolder = (Get-Item $env:TEMP).FullName.TrimEnd('\')
	if ($verbose) {
		Write-Host "Verbose mode enabled."
		Write-Host "Temp folder: $tempFolder"
	}

	foreach ($file in $files) {
		if ($confirm.ToUpper() -ne "N") {
			Write-Host "Processing file: $($file.FullName)"
			$deleteConfirm = Read-Host "Delete this file? (Y/N)"
			if ($deleteConfirm.ToUpper() -ne "Y" -and $deleteConfirm.ToUpper() -ne "A") {
				Write-Host "Skipping file: $($file.FullName)"
				continue
			}	
		}

		if ($deleteConfirm.ToUpper() -eq "A") {
			Write-Host "Auto-confirm enabled. Deleting file: $($file.FullName)"
			$confirm = "N"
			$deleteConfirm = "N"
			continue
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
			if ($verbose) {
				Write-Host "Error: $_ Type: $($_.GetType().FullName)"
				Read-Host "Press Enter to continue..."
			}
			continue
		} catch [System.ArgumentException] {
			Write-Host "Skipping file with invalid path: $($file.FullName)"
			if ($verbose) {
				Write-Host "Error: $_ Type: $($_.GetType().FullName)"
				Read-Host "Press Enter to continue..."
			}
			continue
		}
		catch {
			Write-Host "Failed to delete: $($file.FullName)."
			if ($verbose) {
				Write-Host "Error: $_ Type: $($_.GetType().FullName)"
				Read-Host "Press Enter to continue..."
			}
			continue
		}
	}

	switch ($freedSpace.Bytes) {
		0 { Write-Host "No space freed."}
		{$_ -gt 1000000000} { 
			Write-Host "Freed space: $($freedSpace.GB) GB ($($freedSpace.Bytes) bytes)"
			break
		}
		{$_ -gt 1000000} { 
			Write-Host "Freed space: $($freedSpace.MB) MB ($($freedSpace.Bytes) bytes)"
			break
		}
		{$_ -gt 1000} { 
			Write-Host "Freed space: $($freedSpace.KB) KB ($($freedSpace.Bytes) bytes)"
		}
		default { Write-Host "Freed space: $($freedSpace.Bytes) bytes" }
	}
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

if ($verbose -and $yes) {
	Write-Host "Damn."
	exit
}

$files = Get-ChildItem -Path "$env:TEMP" -Recurse -File -ErrorAction SilentlyContinue

$totalSize = [DataSize]::new(0)
$totalSize.Bytes = Get-FolderSize -Path "$env:TEMP"

if ($files.Count -eq 0) {
	Write-Host "No files found in the temp directory."
	exit
}

$randomFile = $files | Get-Random
$randomFileSize = [DataSize]::new(0)
$randomFileSize.Bytes = $randomFile.Length

if ($verbose) {
	Write-Host "File count: $($files.Count)"
	Write-Host "Files: $($files | ForEach-Object { $_.FullName })"
	Write-Host "Random file: $($randomFile.FullName) (Size: $($randomFileSize.KB) KB)"
}

if ($yes) {
	Write-Host "Auto-confirm enabled. Deleting files without confirmation or information."
	Remove-Files -files $files
	Write-Host "All files in the temp directory have been processed."
	exit
}

switch ($totalSize.Bytes) {
	0 { 
		Write-Host "Only empty files found in the temp directory." 
		exit 
	}
	{ $_ -gt 0.9GB } { 
		Write-Host "Found $(($files.Count)) files in the temp directory, taking up $($totalSize.GB) GB ($($totalSize.KB) KB)" 
		break 
	}
	{ $_ -gt 0.9MB } { 
		Write-Host "Found $(($files.Count)) files in the temp directory, taking up $($totalSize.MB) MB ($($totalSize.Bytes) bytes)" 
		break 
	}
	{ $_ -gt 0.9KB } { 
		Write-Host "Found $(($files.Count)) files in the temp directory, taking up $($totalSize.KB) KB ($($totalSize.Bytes) bytes)" 
	}
	default { 
		Write-Host "Found $(($files.Count)) files in the temp directory, taking up $($totalSize.Bytes) bytes"
	}
}

Write-Host "Example file: $($randomFile.FullName) (Size: $($randomFileSize.KB) KB)"
Write-Host "Delete all temp files? (Y/N)"
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
