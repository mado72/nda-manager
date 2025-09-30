-- Usuários (Clientes e Fornecedores)
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    stellar_public_key TEXT UNIQUE NOT NULL,
    stellar_secret_key TEXT NOT NULL,
    user_type TEXT NOT NULL CHECK (user_type IN ('client', 'supplier')),
    created_at DATETIME NOT NULL
);

-- Processos com informações confidenciais
CREATE TABLE processes (
    id TEXT PRIMARY KEY,
    client_id TEXT NOT NULL,
    title TEXT NOT NULL,
    encrypted_content TEXT NOT NULL,
    encryption_key TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at DATETIME NOT NULL,
    FOREIGN KEY (client_id) REFERENCES users (id)
);

-- Compartilhamentos de processos via Stellar
CREATE TABLE process_shares (
    id TEXT PRIMARY KEY,
    process_id TEXT NOT NULL,
    supplier_public_key TEXT NOT NULL,
    stellar_transaction_hash TEXT NOT NULL,
    shared_at DATETIME NOT NULL,
    FOREIGN KEY (process_id) REFERENCES processes (id)
);

-- Acessos aos processos (para notificações)
CREATE TABLE process_accesses (
    id TEXT PRIMARY KEY,
    process_id TEXT NOT NULL,
    supplier_id TEXT NOT NULL,
    accessed_at DATETIME NOT NULL,
    FOREIGN KEY (process_id) REFERENCES processes (id),
    FOREIGN KEY (supplier_id) REFERENCES users (id)
);

-- Índices para performance
CREATE INDEX idx_processes_client_id ON processes (client_id);
CREATE INDEX idx_process_shares_process_id ON process_shares (process_id);
CREATE INDEX idx_process_accesses_process_id ON process_accesses (process_id);