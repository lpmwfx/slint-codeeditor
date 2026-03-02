// BAppBuilder Runtime — CORE JavaScript entry point
// Klik på 📂 Åbn for at åbne en fil, eller Ctrl+O

const APP_VERSION = "1.0.0";
const DB_NAME = "appdata";

async function initializeApp() {
  const config = await PAL.readState("app.config");

  if (!config) {
    console.log("Første kørsel — opretter defaults");
    await createDefaults();
  }

  PAL.registerAPI({
    "inventory.list": listItems,
    "inventory.add": addItem,
    "inventory.search": searchItems,
  });
}

async function listItems(filter) {
  const query = `
    SELECT id, name, quantity, location
    FROM inventory
    WHERE active = true
    ORDER BY name ASC
  `;
  const results = await PAL.sql(DB_NAME, query);
  PAL.writeState("ui.inventory.items", results);
  return results;
}

async function addItem(item) {
  const { name, quantity, location } = item;

  if (!name || quantity < 0) {
    throw new Error("Ugyldige varedata");
  }

  await PAL.sql(DB_NAME, `
    INSERT INTO inventory (name, quantity, location)
    VALUES (?, ?, ?)
  `, [name, quantity, location]);

  await listItems();
}

function searchItems(term) {
  const normalized = term.toLowerCase().trim();
  return PAL.sql(DB_NAME, `
    SELECT * FROM inventory
    WHERE LOWER(name) LIKE '%' || ? || '%'
  `, [normalized]);
}

async function createDefaults() {
  await PAL.writeState("app.config", {
    version: APP_VERSION,
    created: new Date().toISOString(),
    theme: "default",
  });
  await PAL.writeState("ui.activeTab", "inventory");
}

initializeApp();
