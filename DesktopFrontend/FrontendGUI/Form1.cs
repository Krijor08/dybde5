using System;
using System.Collections.Generic; // Nødvendig for List
using System.Drawing;
using System.Drawing.Drawing2D;
using System.Windows.Forms;
using System.Net.Http;
using System.Text;
using System.Text.Json;

namespace FrontendGUI;

public partial class Form1 : Form
{
    // --- MINNE: En liste som holder styr på hele samtalen ---
    private List<object> chatHistorikk = new List<object>();

    public Form1()
    {
        InitializeComponent();
        SetupMainInterface();
    }

    private void SetupMainInterface()
    {
        this.WindowState = FormWindowState.Maximized;
        this.BackColor = Color.FromArgb(15, 15, 18);
        this.Text = "Main Dashboard - RAHHH";

        // --- 1. KAFFE-MODUS KNAPPEN ---
        Button btnKaffe = new Button();
        btnKaffe.Size = new Size(400, 400);
        btnKaffe.Text = "KAFFE-MODUS";
        btnKaffe.Location = new Point(1000, 100); 
        btnKaffe.BackColor = Color.FromArgb(139, 69, 19);
        btnKaffe.ForeColor = Color.White;
        btnKaffe.Font = new Font("Impact", 35);
        btnKaffe.FlatStyle = FlatStyle.Flat;

        GraphicsPath pathKaffe = new GraphicsPath();
        pathKaffe.AddEllipse(0, 0, btnKaffe.Width, btnKaffe.Height);
        btnKaffe.Region = new Region(pathKaffe);
        btnKaffe.Click += (s, e) => {
            using (LinuxCommandForm terminalVindu = new LinuxCommandForm())
            {
                terminalVindu.ShowDialog();
            }
        };

        // --- 2. AI CHAT INTERFACE ---

        RichTextBox aiOutput = new RichTextBox
        {
            Location = new Point(50, 50),
            Size = new Size(800, 450),
            BackColor = Color.FromArgb(25, 25, 30),
            ForeColor = Color.FromArgb(0, 255, 150),
            Font = new Font("Consolas", 11),
            BorderStyle = BorderStyle.None,
            ReadOnly = true
        };

        TextBox aiInput = new TextBox
        {
            Location = new Point(50, 520),
            Size = new Size(800, 100),
            Multiline = true,
            BackColor = Color.FromArgb(35, 35, 40),
            ForeColor = Color.White,
            Font = new Font("Segoe UI", 12),
            BorderStyle = BorderStyle.FixedSingle
        };

        // --- 3. CLEAR BUTTON (Slik du ba om) ---
        Button btnClear = new Button
        {
            Text = "CLEAR CHAT",
            Location = new Point(50, 630),
            Size = new Size(120, 40),
            BackColor = Color.FromArgb(60, 20, 20),
            ForeColor = Color.White,
            FlatStyle = FlatStyle.Flat
        };

        btnClear.Click += (s, e) => {
            chatHistorikk.Clear(); // Tømmer AI-minnet
            aiOutput.Clear();      // Tømmer skjermen
        };

        // --- SEND LOGIKK MED MINNE ---
        aiInput.KeyDown += async (s, e) => {
            if (e.KeyCode == Keys.Enter && !e.Shift)
            {
                e.SuppressKeyPress = true;
                string prompt = aiInput.Text.Trim();
                if (string.IsNullOrEmpty(prompt)) return;

                aiOutput.AppendText("DEG: " + prompt + "\n");
                aiInput.Clear();

                // Legg til din melding i minnet
                chatHistorikk.Add(new { role = "user", content = prompt });

                try 
                {
                    using var client = new HttpClient();
                    var url = "http://localhost:1234/v1/chat/completions";
                    
                    // Nå sender vi HELE chatHistorikk i stedet for bare den siste meldingen
                    var payload = new { 
                        model = "local-model", 
                        messages = chatHistorikk.ToArray(),
                        temperature = 0.7
                    };

                    var json = JsonSerializer.Serialize(payload);
                    var content = new StringContent(json, Encoding.UTF8, "application/json");
                    
                    var response = await client.PostAsync(url, content);
                    var resString = await response.Content.ReadAsStringAsync();
                    
                    using var doc = JsonDocument.Parse(resString);
                    string svar = doc.RootElement.GetProperty("choices")[0].GetProperty("message").GetProperty("content").GetString();
                    
                    // Legg AI-ens svar inn i minnet også!
                    chatHistorikk.Add(new { role = "assistant", content = svar });
                    
                    aiOutput.AppendText("AI: " + svar + "\n\n");
                }
                catch 
                {
                    aiOutput.AppendText("SYSTEM: Feil ved tilkobling.\n");
                }
                
                aiOutput.SelectionStart = aiOutput.Text.Length;
                aiOutput.ScrollToCaret();
            }
        };

        // --- 4. DE RØDE KNAPPENE ---
        Button red1 = new Button { Text = "RED 1", Size = new Size(150, 80), Location = new Point(2100, 600), BackColor = Color.Red, ForeColor = Color.White, FlatStyle = FlatStyle.Flat, Font = new Font("Arial", 12, FontStyle.Bold) };
        red1.Click += (s, e) => { new RedButton1().Show(); };

        Button red2 = new Button { Text = "RED 2", Size = new Size(150, 80), Location = new Point(2300, 600), BackColor = Color.DarkRed, ForeColor = Color.White, FlatStyle = FlatStyle.Flat, Font = new Font("Arial", 12, FontStyle.Bold) };
        red2.Click += (s, e) => { new RedButton2().Show(); };

        Button red3 = new Button { Text = "RED 3", Size = new Size(150, 80), Location = new Point(2500, 600), BackColor = Color.Crimson, ForeColor = Color.White, FlatStyle = FlatStyle.Flat, Font = new Font("Arial", 12, FontStyle.Bold) };
        red3.Click += (s, e) => { new RedButton3().Show(); };

        // Legg alle kontrollene til i vinduet
        this.Controls.Add(btnKaffe);
        this.Controls.Add(aiOutput);
        this.Controls.Add(aiInput);
        this.Controls.Add(btnClear);
        this.Controls.Add(red1);
        this.Controls.Add(red2);
        this.Controls.Add(red3);
    }
}