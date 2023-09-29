SELECT aschanges.id, ashes.ash, ante.ash ante, aschanges.time,
       aschanges.sigma, product.ash product, aschanges.alias, aschanges.rate
FROM aschanges JOIN ashes ON aschanges.ash_id = ashes.id
JOIN ashes ante ON aschanges.ante_id = ante.id
JOIN ashes product ON aschanges.product_id = product.id