# auth-service

Service d'authentification centralisé. Il génère et valide des JWT — les autres services n'ont jamais besoin de connaître le secret.

---

## Démarrage rapide

```bash
# Copier la config
cp .env.example .env

# Lancer le serveur
cargo run
```

Le serveur démarre sur `http://localhost:3000`.

---

## Variables d'environnement

| Variable | Défaut | Description |
|---|---|---|
| `HOST` | `0.0.0.0` | Adresse d'écoute |
| `PORT` | `3000` | Port d'écoute |
| `JWT_SECRET` | `dev-secret` | Secret de signature des JWT — **à changer en prod** |

> ⚠️ Ne jamais commiter `.env`. Utiliser `.env.example` comme référence.

---

## Endpoints

### `GET /health`

Vérifie que le service tourne.

**Réponse**
```json
{ "status": "ok" }
```

---

### `POST /auth/login`

Authentifie un utilisateur et retourne un JWT.

> ⚠️ Actuellement en mode **dummy** : accepte n'importe quel email/password.

**Corps de la requête**
```json
{
  "email": "user@example.com",
  "password": "secret"
}
```

**Réponse (200)**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "email": "user@example.com"
}
```

---

### `GET /auth/me`

Retourne les informations de l'utilisateur connecté.

**Header requis**
```
Authorization: Bearer <token>
```

**Réponse (200)**
```json
{
  "email": "user@example.com"
}
```

**Réponse (401) — token manquant**
```json
{ "error": "Missing token" }
```

**Réponse (401) — token invalide ou expiré**
```json
{ "error": "Invalid or expired token" }
```

---

### `POST /auth/introspect`

Vérifie la validité d'un token. C'est l'endpoint utilisé par les autres services.

**Corps de la requête**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

**Réponse — token valide**
```json
{
  "active": true,
  "email": "user@example.com"
}
```

**Réponse — token invalide ou expiré**
```json
{
  "active": false,
  "email": null
}
```

---

## Intégration depuis un autre service

Le flow complet :

```
1. Client → POST /auth/login          → reçoit un JWT
2. Client → GET  /votre-service/data  → envoie le JWT dans le header
3. Votre service → POST /auth/introspect → vérifie le JWT
4. Votre service → traite ou rejette la requête
```

Le JWT voyage toujours dans le header `Authorization` :

```
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

---

### Exemple en Rust

```rust
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct IntrospectResponse {
    active: bool,
    email: Option<String>,
}

async fn verify_token(token: &str) -> Result<String, String> {
    let client = Client::new();

    let res = client
        .post("http://auth-service:3000/auth/introspect")
        .json(&serde_json::json!({ "token": token }))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<IntrospectResponse>()
        .await
        .map_err(|e| e.to_string())?;

    if res.active {
        Ok(res.email.unwrap_or_default())
    } else {
        Err("Token invalide".to_string())
    }
}

// Dans un handler Axum protégé :
async fn protected_handler(headers: HeaderMap) -> Result<Json<...>, MyError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(MyError::Unauthorized)?;

    let email = verify_token(token).await
        .map_err(|_| MyError::Unauthorized)?;

    // email contient l'utilisateur identifié
    Ok(Json(...))
}
```

---

### Exemple en Python (FastAPI)

```python
import httpx
from fastapi import FastAPI, Header, HTTPException

app = FastAPI()

async def verify_token(token: str) -> str:
    async with httpx.AsyncClient() as client:
        res = await client.post(
            "http://auth-service:3000/auth/introspect",
            json={"token": token}
        )
        data = res.json()

    if not data["active"]:
        raise HTTPException(status_code=401, detail="Token invalide")

    return data["email"]

@app.get("/data")
async def get_data(authorization: str = Header(...)):
    # authorization = "Bearer eyJ0eXAi..."
    token = authorization.removeprefix("Bearer ")
    email = await verify_token(token)

    # email contient l'utilisateur identifié
    return {"message": f"Bonjour {email}"}
```

---

### Exemple en Node.js (Express)

```javascript
const express = require('express');
const axios = require('axios');

const app = express();

async function verifyToken(token) {
  const res = await axios.post('http://auth-service:3000/auth/introspect', {
    token
  });

  if (!res.data.active) {
    throw new Error('Token invalide');
  }

  return res.data.email;
}

// Middleware réutilisable sur toutes les routes protégées
async function requireAuth(req, res, next) {
  try {
    const auth = req.headers['authorization'] || '';
    const token = auth.replace('Bearer ', '');

    req.userEmail = await verifyToken(token);
    next();
  } catch {
    res.status(401).json({ error: 'Non autorisé' });
  }
}

// Route protégée
app.get('/data', requireAuth, (req, res) => {
  res.json({ message: `Bonjour ${req.userEmail}` });
});
```

---

### Exemple en PHP

```php
function verifyToken(string $token): ?string
{
    $response = file_get_contents(
        'http://auth-service:3000/auth/introspect',
        false,
        stream_context_create([
            'http' => [
                'method'  => 'POST',
                'header'  => 'Content-Type: application/json',
                'content' => json_encode(['token' => $token]),
            ]
        ])
    );

    $data = json_decode($response, true);

    return $data['active'] ? $data['email'] : null;
}

// Dans un endpoint protégé
$authHeader = $_SERVER['HTTP_AUTHORIZATION'] ?? '';
$token = str_replace('Bearer ', '', $authHeader);
$email = verifyToken($token);

if (!$email) {
    http_response_code(401);
    echo json_encode(['error' => 'Non autorisé']);
    exit;
}

// $email contient l'utilisateur identifié
```

---

## Structure du projet

```
auth-service/
├── src/
│   ├── main.rs       # Démarrage du serveur et déclaration des routes
│   ├── models.rs     # Structs de données (LoginRequest, Claims, etc.)
│   ├── handlers.rs   # Logique des endpoints
│   └── errors.rs     # Gestion des erreurs → réponses HTTP propres
├── .env              # Variables d'environnement (ne pas commiter)
├── .env.example      # Template des variables (à commiter)
└── Cargo.toml        # Dépendances
```

---

## Codes d'erreur

| Code | Signification |
|---|---|
| `200` | Succès |
| `401` | Token manquant, invalide ou expiré |
| `500` | Erreur interne du serveur |