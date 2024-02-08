
READ_TRANSACTION_SQL = """
SELECT valor, tipo, descricao, realizada_em
FROM transacoes t
JOIN clientes c on c.id = t.cliente_id
WHERE c.id = $1
ORDER BY t.realizada_em DESC
LIMIT 10
"""

READ_ACCOUNT_STATEMENT_SQL = """
SELECT c.limite as limite, NOW() as data_extrato, SUM(s.valor) OVER ()
FROM clientes c 
JOIN saldos s on c.id = s.cliente_id 
WHERE c.id = $1
"""

INSERT_TRANSACTION_SQL = "INSERT INTO transacoes (cliente_id, valor, tipo, descricao) VALUES ($1, $2, $3, $4)"
