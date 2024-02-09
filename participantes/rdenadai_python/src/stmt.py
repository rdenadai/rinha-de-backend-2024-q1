READ_TRANSACTION_SQL = """
SELECT valor, tipo, descricao, realizada_em
FROM transacoes t
JOIN clientes c on c.id = t.cliente_id
WHERE c.id = $1
ORDER BY t.realizada_em DESC
LIMIT 10;
"""

READ_ACCOUNT_STATEMENT_SQL = """
SELECT c.limite as limite, NOW() as data_extrato, s.valor as total
FROM clientes c 
JOIN saldos s on c.id = s.cliente_id 
WHERE c.id = $1;
"""

UPDATE_BALANCE_SQL = "SELECT limite, saldo FROM atualiza_saldo($1, $2, $3, $4);"
