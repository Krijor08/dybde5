using System;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.Windows.Forms;

namespace FrontendGUI
{
    public partial class Form1 : Form
    {
        public Form1()
        {
            // Vindusoppsett
            this.WindowState = FormWindowState.Maximized;
            this.BackColor = Color.FromArgb(15, 15, 18);
            this.Text = "Frontend Kontrollpanel";

            // --- 1. KAFFE-MODUS (STOR OG RUND) ---
            Button btnKaffe = new Button();
            btnKaffe.Size = new Size(500, 500);
            btnKaffe.Text = "KAFFE-MODUS";
            
            // JUSTER DENNE FOR POSISJON
            btnKaffe.Location = new Point(200, 350); 
            
            btnKaffe.BackColor = Color.FromArgb(139, 69, 19);
            btnKaffe.ForeColor = Color.White;
            btnKaffe.Font = new Font("Impact", 45);
            btnKaffe.FlatStyle = FlatStyle.Flat;
            btnKaffe.FlatAppearance.BorderSize = 0;

            GraphicsPath pathKaffe = new GraphicsPath();
            pathKaffe.AddEllipse(0, 0, btnKaffe.Width, btnKaffe.Height);
            btnKaffe.Region = new Region(pathKaffe);
            btnKaffe.Click += (s, e) => MessageBox.Show("Kaffen er klar! ☕");


            // --- 2. RED BUTTON (LITEN OG RUND) ---
            Button btnRed = new Button();
            btnRed.Size = new Size(80, 80);
            btnRed.Text = "RED";
            
            // JUSTER DENNE FOR POSISJON (f.eks. 1800, 50 for topp høyre)
            btnRed.Location = new Point(2000, 50); 
            
            btnRed.Font = new Font("Impact", 12);
            btnRed.BackColor = Color.FromArgb(220, 20, 60);
            btnRed.ForeColor = Color.White;
            btnRed.FlatStyle = FlatStyle.Flat;
            btnRed.FlatAppearance.BorderSize = 0;

            GraphicsPath pathRed = new GraphicsPath();
            pathRed.AddEllipse(0, 0, btnRed.Width, btnRed.Height);
            btnRed.Region = new Region(pathRed);

            btnRed.Click += (s, e) => {
                using (GameMenuForm meny = new GameMenuForm()) { meny.ShowDialog(); }
            };


            // Legg til i vinduet
            this.Controls.Add(btnKaffe);
            this.Controls.Add(btnRed);
        }
    }

    public class GameMenuForm : Form
    {
        public GameMenuForm()
        {
            this.Size = new Size(400, 550);
            this.Text = "Big Red Button Menu";
            this.StartPosition = FormStartPosition.CenterParent;
            this.BackColor = Color.FromArgb(30, 30, 35);
            this.FormBorderStyle = FormBorderStyle.FixedDialog;

            string[] redButtons = { "Red Button 1", "Red Button 2", "Red Button 3" };
            for (int i = 0; i < redButtons.Length; i++)
            {
                Button b = new Button {
                    Text = redButtons[i],
                    Size = new Size(250, 70),
                    Location = new Point(75, 60 + (i * 100)),
                    BackColor = Color.FromArgb(60, 60, 70),
                    ForeColor = Color.Cyan,
                    Font = new Font("Segoe UI", 14, FontStyle.Bold),
                    FlatStyle = FlatStyle.Flat
                };
                
                int id = i;
                b.Click += (s, e) => {
                    if (id == 0) new RedButton1().Show();
                    if (id == 1) new RedButton2().Show();
                    if (id == 2) new RedButton3().Show();
                    this.Close(); 
                };
                this.Controls.Add(b);
            }
        }
    }
}