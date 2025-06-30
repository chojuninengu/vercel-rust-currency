const { exec } = require('child_process');
const util = require('util');
const execAsync = util.promisify(exec);

module.exports = async (req, res) => {
  const { path } = req;
  
  try {
    if (path === '/api/currencies') {
      const { stdout } = await execAsync('cargo run --bin currencies');
      res.json(JSON.parse(stdout));
    } else if (path === '/api/convert') {
      const { from, to, amount } = req.query;
      const { stdout } = await execAsync(`cargo run --bin convert -- --from ${from} --to ${to} --amount ${amount}`);
      res.json(JSON.parse(stdout));
    } else {
      res.status(404).json({ error: 'Not found' });
    }
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
}; 