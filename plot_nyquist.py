import csv
import matplotlib.pyplot as plt

# Read data from CSV file
frequencies = []
real_impedance = []
imag_impedance = []

with open('ecm_impedance.csv') as csv_file:
    csv_reader = csv.reader(csv_file)
    next(csv_reader)  # Skip header row
    for row in csv_reader:
        frequencies.append(float(row[0]))
        real_impedance.append(float(row[1]))
        imag_impedance.append(float(row[2]))

# Plot real and imaginary components of impedance
fig, ax = plt.subplots()
ax.scatter(real_impedance, [-z for z in imag_impedance], s=10)
ax.set_xlabel('Real Impedance (Ohm)')
ax.set_ylabel('-Imaginary Impedance (Ohm)')
ax.legend()
plt.draw()
fig.savefig('impedance_plot.png', dpi=300, bbox_inches='tight')
