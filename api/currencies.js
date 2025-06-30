const fetch = require('node-fetch');

module.exports = async (req, res) => {
  const API_KEY = process.env.EXCHANGE_RATE_API_KEY;
  
  if (!API_KEY) {
    res.status(500).json({ error: 'API key not configured' });
    return;
  }

  const url = `https://v6.exchangerate-api.com/v6/${API_KEY}/latest/USD`;

  try {
    const response = await fetch(url);
    const data = await response.json();
    const currencies = Object.keys(data.conversion_rates).sort();

    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
    res.status(200).json(currencies);
  } catch (error) {
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.status(500).json({ error: error.message });
  }
}; 