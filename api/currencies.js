module.exports = async (req, res) => {
  const API_KEY = '7ed63c331cfb0a7ef831a95a';
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