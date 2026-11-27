# Huellitas: donaciones con Solana

Alcancía web para un refugio de animales **ficticio**. Montos en pesos mexicanos (MXN) y pagos en Solana **devnet** (sin dinero real). Proyecto escolar de demostración.

## Funciones
- Montos rápidos o personalizados
- Donación con QR (Solana Pay) o con Phantom
- Verificación del pago en la blockchain
- Barra de avance hacia la meta e historial

## Tecnologías
HTML, CSS, JavaScript, `@solana/web3.js`, `qrcodejs`, Rust y Anchor.

## Programa en Solana
`programa/lib.rs` contiene la instrucción `donar`, que transfiere la donación y emite el evento `DonacionRecibida`. La página la usa con Phantom cuando `PROGRAM_ID` está configurado en `index.html`. El QR hace una transferencia directa.

## Uso
1. En Phantom activa Testnet Mode y elige Solana Devnet.
2. En Configuración pega la wallet receptora (debe tener algo de SOL de prueba).
3. Dona desde otra cuenta con SOL de devnet.

## Limitaciones
Prototipo: el avance y el historial se guardan solo en el navegador, y una donación pendiente se pierde al recargar.
