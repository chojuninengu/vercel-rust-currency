const fetch = require('node-fetch');

module.exports = async (req, res) => {
  const API_KEY = process.env.EXCHANGE_RATE_API_KEY;
  
  if (!API_KEY) {
    res.status(500).json({ error: 'API key not configured' });
    return;
  }

  const { from, to, amount } = req.query;

  if (!from || !to || !amount) {
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.status(400).json({ error: 'Missing required parameters' });
    return;
  }

  const url = `https://v6.exchangerate-api.com/v6/${API_KEY}/latest/${from}`;

  try {
    const response = await fetch(url);
    const data = await response.json();
    const rate = data.conversion_rates[to];

    if (!rate) {
      res.setHeader('Access-Control-Allow-Origin', '*');
      res.status(400).json({ error: 'Invalid currency code' });
      return;
    }

    const result = {
      from,
      to,
      amount: parseFloat(amount),
      result: parseFloat(amount) * rate,
      rate,
      timestamp: new Date().toISOString(),
    };

    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
    res.status(200).json(result);
  } catch (error) {
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.status(500).json({ error: error.message });
  }
}; 