# Definir la ruta base
$basePath = "C:\Users\david\OneDrive\Documents\arbitrage\dominio"

# Crear directorio raíz si no existe
if (-not (Test-Path $basePath)) {
    New-Item -Path $basePath -ItemType Directory
}

# Crear archivo Cargo.toml si no existe
if (-not (Test-Path "$basePath\Cargo.toml")) {
    New-Item -Path "$basePath\Cargo.toml" -ItemType File
}

# Crear directorio src si no existe
$srcPath = "$basePath\src"
if (-not (Test-Path $srcPath)) {
    New-Item -Path $srcPath -ItemType Directory
}

# Crear archivos en src/
$srcFiles = @(
    "lib.rs",
    "prelude.rs"
)
foreach ($file in $srcFiles) {
    $filePath = Join-Path $srcPath $file
    if (-not (Test-Path $filePath)) {
        New-Item -Path $filePath -ItemType File
    }
}

# Crear directorio errors/ y sus archivos
$errorsPath = "$srcPath\errors"
if (-not (Test-Path $errorsPath)) {
    New-Item -Path $errorsPath -ItemType Directory
}
$errorsFiles = @(
    "mod.rs",
    "domain_error.rs"
)
foreach ($file in $errorsFiles) {
    $filePath = Join-Path $errorsPath $file
    if (-not (Test-Path $filePath)) {
        New-Item -Path $filePath -ItemType File
    }
}

# Crear directorio value_objects/ y sus archivos
$voPath = "$srcPath\value_objects"
if (-not (Test-Path $voPath)) {
    New-Item -Path $voPath -ItemType Directory
}
$voFiles = @(
    "mod.rs",
    "prelude.rs",
    "money.rs"
)
foreach ($file in $voFiles) {
    $filePath = Join-Path $voPath $file
    if (-not (Test-Path $filePath)) {
        New-Item -Path $filePath -ItemType File
    }
}

# Crear directorio entities/ y sus archivos
$entitiesPath = "$srcPath\entities"
if (-not (Test-Path $entitiesPath)) {
    New-Item -Path $entitiesPath -ItemType Directory
}
$entitiesFiles = @(
    "mod.rs",
    "order.rs",
    "builders.rs"
)
foreach ($file in $entitiesFiles) {
    $filePath = Join-Path $entitiesPath $file
    if (-not (Test-Path $filePath)) {
        New-Item -Path $filePath -ItemType File
    }
}

# Crear directorio aggregates/ y sus archivos
$aggregatesPath = "$srcPath\aggregates"
if (-not (Test-Path $aggregatesPath)) {
    New-Item -Path $aggregatesPath -ItemType Directory
}
$aggregatesFiles = @(
    "mod.rs",
    "portfolio.rs",
    "strategy.rs"
)
foreach ($file in $aggregatesFiles) {
    $filePath = Join-Path $aggregatesPath $file
    if (-not (Test-Path $filePath)) {
        New-Item -Path $filePath -ItemType File
    }
}

# Crear directorio services/domain/ y sus archivos
$servicesPath = "$srcPath\services\domain"
if (-not (Test-Path $servicesPath)) {
    New-Item -Path $servicesPath -ItemType Directory -Force
}
$servicesFiles = @(
    "mod.rs",
    "pricing.rs",
    "risk.rs"
)
foreach ($file in $servicesFiles) {
    $filePath = Join-Path $servicesPath $file
    if (-not (Test-Path $filePath)) {
        New-Item -Path $filePath -ItemType File
    }
}

# Crear directorio specifications/ si no existe
$specPath = "$srcPath\specifications"
if (-not (Test-Path $specPath)) {
    New-Item -Path $specPath -ItemType Directory
}

# Crear directorio policies/ si no existe
$policiesPath = "$srcPath\policies"
if (-not (Test-Path $policiesPath)) {
    New-Item -Path $policiesPath -ItemType Directory
}

# Crear directorio events/ y sus archivos
$eventsPath = "$srcPath\events"
if (-not (Test-Path $eventsPath)) {
    New-Item -Path $eventsPath -ItemType Directory
}
$eventsFiles = @(
    "mod.rs",
    "order_filled.rs",
    "dispatcher.rs"
)
foreach ($file in $eventsFiles) {
    $filePath = Join-Path $eventsPath $file
    if (-not (Test-Path $filePath)) {
        New-Item -Path $filePath -ItemType File
    }
}

# Crear directorio repositories/ y sus archivos
$reposPath = "$srcPath\repositories"
if (-not (Test-Path $reposPath)) {
    New-Item -Path $reposPath -ItemType Directory
}
$reposFiles = @(
    "mod.rs",
    "order_repository.rs"
)
foreach ($file in $reposFiles) {
    $filePath = Join-Path $reposPath $file
    if (-not (Test-Path $filePath)) {
        New-Item -Path $filePath -ItemType File
    }
}

# Crear directorio contracts/ y sus archivos
$contractsPath = "$srcPath\contracts"
if (-not (Test-Path $contractsPath)) {
    New-Item -Path $contractsPath -ItemType Directory
}
$contractsFiles = @(
    "mod.rs",
    "price_feed.rs"
)
foreach ($file in $contractsFiles) {
    $filePath = Join-Path $contractsPath $file
    if (-not (Test-Path $filePath)) {
        New-Item -Path $filePath -ItemType File
    }
}

# Crear directorio tests/ si no existe
$testsPath = "$srcPath\tests"
if (-not (Test-Path $testsPath)) {
    New-Item -Path $testsPath -ItemType Directory
}

Write-Host "Estructura de directorios y archivos creada en '$basePath' (archivos existentes no se modificaron)."