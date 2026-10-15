# ---AcuerdoClaro---

Acuerdos verificables entre freelancers y clientes en **Stellar Testnet**. El texto del acuerdo queda fuera de la cadena; en Stellar se registra su huella SHA-256, las partes, el estado y la fecha de aceptación.

## Contrato (Soroban)
```bash
cd contract
cargo test
stellar contract build
stellar keys generate dev --network testnet --fund
stellar contract deploy --wasm target/wasm32v1-none/release/acuerdo_claro.wasm \
  --source dev --network testnet        # → guarda la dirección C…
```
Funciones: `create`, `accept`, `cancel`, `get`, `verify`, `count`.

## Frontend
```bash
cd frontend && npx serve .     # abrir http://localhost:3000
```
Pega la dirección C…, conecta Freighter (en Testnet) y crea/acepta un acuerdo. El enlace a Stellar Expert aparece al terminar la transacción.

## Entrega APEX
- Contract ID: `C…`
- Tx: `https://stellar.expert/explorer/testnet/tx/<hash>`
- Repo con 10+ commits

## Commits para interactuar
1. chore: estructura inicial y .gitignore
2. feat(contract): structs Agreement/Status y errores
3. feat(contract): create
4. feat(contract): accept
5. feat(contract): cancel
6. feat(contract): get, verify y count
7. test(contract): pruebas de flujo y permisos
8. feat(frontend): maqueta y estilos
9. feat(frontend): conexión con Freighter
10. feat(frontend): invocar create/accept y link a Stellar Expert
11. docs: README con dirección del contrato y tx

## R0D :v
