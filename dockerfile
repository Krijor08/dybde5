# STAGE 1: Rust (Denne fungerte fint i loggen din!)
FROM rust:1.93.1 AS rust-builder
WORKDIR /app
COPY . .
RUN cargo build --release

# STAGE 2: .NET Builder (HER LÅ FEILEN)
FROM mcr.microsoft.com/dotnet/sdk:10.0 AS dotnet-builder
WORKDIR /app
COPY . .
# Vi legger til '-p:EnableWindowsTargeting=true' for å lure Linux til å bygge WinForms
RUN dotnet publish "DesktopFrontend/FrontendGUI/FrontendGUI.csproj" \
    -c Release \
    -o /app/publish \
    -p:EnableWindowsTargeting=true

# STAGE 3: Final Runtime
FROM mcr.microsoft.com/dotnet/runtime:10.0
WORKDIR /app

# Installer Bash + verktøyene scriptene dine trenger (f.eks. iproute2 for ip-kommandoer)
RUN apt-get update && apt-get install -y \
    bash \
    iproute2 \
    iputils-ping \
    && rm -rf /var/lib/apt/lists/*

COPY --from=rust-builder /app/target/release/dybde5 /app/rust-backend
COPY --from=dotnet-builder /app/publish .
COPY scripts/ /app/scripts/
RUN chmod +x /app/scripts/*

ENTRYPOINT ["dotnet", "FrontendGUI.dll"]