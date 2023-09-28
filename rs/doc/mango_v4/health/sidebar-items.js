window.SIDEBAR_ITEMS = {"enum":[["HealthType","There are three types of health:"]],"fn":[["compute_health","Compute health with an arbitrary AccountRetriever"],["compute_health_from_fixed_accounts","Computes health for a mango account given a set of account infos"],["new_fixed_order_account_retriever",""],["new_health_cache","Generate a HealthCache for an account and its health accounts."],["new_health_cache_skipping_bad_oracles","Generate a special HealthCache for an account and its health accounts where nonnegative token positions for bad oracles are skipped."],["spot_amount_given_for_health_zero","How much of a token can be gained before health increases to zero?"],["spot_amount_taken_for_health_zero","How much of a token can be taken away before health decreases to zero?"]],"struct":[["FixedOrderAccountRetriever","Assumes the account infos needed for the health computation follow a strict order."],["HealthCache","Store information needed to compute account health"],["PerpInfo","Stores information about perp market positions and their open orders."],["Prices","Information about prices for a bank or perp market."],["ScannedBanksAndOracles",""],["ScanningAccountRetriever","Takes a list of account infos containing"],["Serum3Info","Information about reserved funds on Serum3 open orders accounts."],["TokenBalance","Temporary value used during health computations"],["TokenInfo",""],["TokenMaxReserved",""]],"trait":[["AccountRetriever","This trait abstracts how to find accounts needed for the health computation."]]};