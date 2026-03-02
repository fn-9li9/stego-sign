CREATE SCHEMA IF NOT EXISTS app;

-- ENUM (NO usar IF NOT EXISTS aquí)
CREATE TYPE app.document_status AS ENUM ('VALID', 'TAMPERED', 'UNREGISTERED', 'INVALID');

CREATE TABLE IF NOT EXISTS app.documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    filename TEXT NOT NULL,
    hash_sha256 TEXT NOT NULL,
    signature TEXT NOT NULL,
    author TEXT NOT NULL,
    object_id UUID NOT NULL REFERENCES files.objects (id) ON DELETE RESTRICT,
    signed_at TIMESTAMPTZ NOT NULL DEFAULT now (),
    status app.document_status NOT NULL DEFAULT 'VALID',
    metadata JSONB
);

CREATE INDEX IF NOT EXISTS idx_documents_hash ON app.documents (hash_sha256);

CREATE INDEX IF NOT EXISTS idx_documents_author ON app.documents (author);

CREATE INDEX IF NOT EXISTS idx_documents_signed_at ON app.documents (signed_at DESC);

CREATE INDEX IF NOT EXISTS idx_documents_status ON app.documents (status);

CREATE TABLE IF NOT EXISTS app.audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    document_id UUID REFERENCES app.documents (id) ON DELETE SET NULL,
    checked_at TIMESTAMPTZ NOT NULL DEFAULT now (),
    result app.document_status NOT NULL,
    checked_hash TEXT,
    details JSONB NOT NULL DEFAULT '{}',
    action TEXT NOT NULL DEFAULT 'VERIFY'
);

CREATE INDEX IF NOT EXISTS idx_audit_document_id ON app.audit_log (document_id);

CREATE INDEX IF NOT EXISTS idx_audit_checked_at ON app.audit_log (checked_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_result ON app.audit_log (result);

CREATE INDEX IF NOT EXISTS idx_audit_action ON app.audit_log (action);