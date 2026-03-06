using System;
using System.Drawing;
using System.Windows.Forms;
using System.Drawing.Drawing2D;

namespace FrontendGUI; // Endret fra MinGuiApp til FrontendGUI

public partial class Form1 : Form
{
    public Form1()
    {
        // --- FULLSKJERM OPPSETT ---
        this.WindowState = FormWindowState.Maximized;
        this.TopMost = true;
        this.Text = "Frontend Kontrollpanel";

        // 1. Den "Store" knappen (minKnapp - Nuclear / Feil knapp)
        Button minKnapp = new Button();
        minKnapp.Size = new Size(500, 500);
        minKnapp.Text = "BIG RED BUTTON!";
        minKnapp.Location = new Point(200, 400);
        minKnapp.BackColor = Color.Red;
        minKnapp.ForeColor = Color.White;
        minKnapp.Font = new Font("Arial", 40, FontStyle.Bold);
        minKnapp.FlatStyle = FlatStyle.Flat; 
        minKnapp.FlatAppearance.BorderSize = 0;
        
        GraphicsPath form = new GraphicsPath();
        form.AddEllipse(0, 0, minKnapp.Width, minKnapp.Height); 
        minKnapp.Region = new Region(form);

        minKnapp.Click += (sender, e) => {
            try 
            {
                Form bildePopup = new Form();
                bildePopup.Text = "Nuclear Launch button Pressed!";
                bildePopup.Size = new Size(1000, 1000);
                bildePopup.StartPosition = FormStartPosition.CenterScreen;

                PictureBox pb = new PictureBox();
                pb.Dock = DockStyle.Fill;
                pb.Image = Image.FromFile("wrong_button.jpg");
                pb.SizeMode = PictureBoxSizeMode.Zoom;

                bildePopup.Controls.Add(pb);

                // Timer som lukker hele programmet etter 5 sekunder
                System.Windows.Forms.Timer lukkTimer = new System.Windows.Forms.Timer();
                lukkTimer.Interval = 5000; 
                lukkTimer.Tick += (s, ev) => {
                    lukkTimer.Stop();
                    Application.Exit(); 
                };
                lukkTimer.Start();

                bildePopup.ShowDialog();
            }
            catch (Exception ex)
            {
                MessageBox.Show("Feil: " + ex.Message);
            }
        };

        // 2. Den "Store" knappen (minKnapp2 - Coffee)
        Button minKnapp2 = new Button();
        minKnapp2.Size = new Size(500, 500);
        minKnapp2.Text = "BIG RED BUTTON!";
        minKnapp2.Location = new Point(1000, 400);
        minKnapp2.BackColor = Color.Red;
        minKnapp2.ForeColor = Color.White;
        minKnapp2.Font = new Font("Arial", 40, FontStyle.Bold);
        minKnapp2.FlatStyle = FlatStyle.Flat; 
        minKnapp2.FlatAppearance.BorderSize = 0;
        
        GraphicsPath form2 = new GraphicsPath();
        form2.AddEllipse(0, 0, minKnapp2.Width, minKnapp2.Height); 
        minKnapp2.Region = new Region(form2);

        minKnapp2.Click += (sender, e) => {
            try 
            {
                Form bildePopup = new Form();
                bildePopup.Text = "Coffee button Pressed!";
                bildePopup.Size = new Size(1000, 1000);
                bildePopup.StartPosition = FormStartPosition.CenterScreen;

                PictureBox pb = new PictureBox();
                pb.Dock = DockStyle.Fill;
                pb.Image = Image.FromFile("coffee_button.jpg");
                pb.SizeMode = PictureBoxSizeMode.Zoom;

                bildePopup.Controls.Add(pb);
                bildePopup.ShowDialog();
            }
            catch (Exception ex)
            {
                MessageBox.Show("Feil: " + ex.Message);
            }
        };

        // Legg til kontrollene
        this.Controls.Add(minKnapp);
        this.Controls.Add(minKnapp2);
    }
}