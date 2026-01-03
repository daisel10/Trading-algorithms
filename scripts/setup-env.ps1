#!/usr/bin/env pwsh
# Setup script for KAIRÓS environment configuration
# This script copies all .env.example files to .env files

Write-Host "🚀 KAIRÓS Environment Setup" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""

$envFiles = @(
    @{Source = ".env.example"; Dest = ".env"; Name = "Global Configuration"},
    @{Source = "apps/kairos-api/.env.example"; Dest = "apps/kairos-api/.env"; Name = "Java API Configuration"},
    @{Source = "apps/kairos-core/.env.example"; Dest = "apps/kairos-core/.env"; Name = "Rust Core Configuration"},
    @{Source = "apps/kairos-web/.env.example"; Dest = "apps/kairos-web/.env"; Name = "Angular Web Configuration"}
)

$created = 0
$skipped = 0

foreach ($file in $envFiles) {
    Write-Host "📄 Processing: $($file.Name)" -ForegroundColor Yellow
    
    if (Test-Path $file.Dest) {
        Write-Host "   ⚠️  Already exists: $($file.Dest)" -ForegroundColor Gray
        Write-Host "   Skipping (use -Force to overwrite)" -ForegroundColor Gray
        $skipped++
    }
    elseif (!(Test-Path $file.Source)) {
        Write-Host "   ❌ Template not found: $($file.Source)" -ForegroundColor Red
    }
    else {
        Copy-Item -Path $file.Source -Destination $file.Dest
        Write-Host "   ✅ Created: $($file.Dest)" -ForegroundColor Green
        $created++
    }
    Write-Host ""
}

Write-Host "================================" -ForegroundColor Cyan
Write-Host "Summary:" -ForegroundColor Cyan
Write-Host "  Created: $created files" -ForegroundColor Green
Write-Host "  Skipped: $skipped files" -ForegroundColor Yellow
Write-Host ""

if ($created -gt 0) {
    Write-Host "⚠️  IMPORTANT: Update the .env files with your actual credentials!" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "  1. Edit .env files and add your API keys and passwords"
    Write-Host "  2. Run: docker compose -f infrastructure/docker-compose.yml up --build"
    Write-Host ""
}

if ($skipped -gt 0) {
    Write-Host "💡 To overwrite existing files, delete them first or modify the script." -ForegroundColor Gray
    Write-Host ""
}
