param(
    [string]$TargetDirectory = "."
)

if (-Not (Test-Path -Path $TargetDirectory)) {
    Write-Host "The specified directory '$TargetDirectory' does not exist." -ForegroundColor Red
    exit
}

$items = Get-ChildItem -Path $TargetDirectory -Recurse

Write-Host "`nRecursively listing contents of: $TargetDirectory`n" -ForegroundColor Cyan
Write-Host ("{0,-30} {1,-60} {2,-10}" -f "Name", "Path", "Type") -ForegroundColor Yellow
Write-Host ("-" * 100)

foreach ($item in $items) {
    $type = if ($item.PSIsContainer) { "Directory" } else { "File" }
    Write-Host ("{0,-30} {1,-60} {2,-10}" -f $item.Name, $item.FullName, $type)
}

Write-Host "`nListing complete.`n" -ForegroundColor Green
