#!/opt/anaconda3/bin/python

#%% Imports
import os
import pandas as pd
import matplotlib.pyplot as plt
os.environ["PATH"] += os.pathsep + '/usr/local/texlive/2021/bin/universal-darwin'
plt.rcParams['font.family'] = 'Avenir'

#%% Settings
#---colors
color1 = '#1F77B4'
color2 = '#FF7F0E'
#---figure size
fig_size = (5,4)
#---font sizes
title_font_size = 20
x_font_size = 16
y_font_size = 16
legend_font_size = 10
x_tick_size = 14
#---line widths
line_width = 3
dash_width = 1
#---border
border_width_x_y = 2
border_width_upper_right = 2
#---ticks
tick_width = 2
tick_length = 6

#%% Functions
def plot_loglog(x,y,in_color):

    # Load the CSV into a pandas DataFrame
    df = pd.read_csv('target/performance.csv')

    # Plot as log log plot
    plt.figure(figsize=fig_size)
    plt.loglog(df[x], df[y], linewidth=line_width, color=in_color)

    # Labeling the axes (optional)
    plt.xlabel(x, fontsize=x_font_size)
    plt.ylabel(y, fontsize=y_font_size)
    plt.title('minimax + egg performance', fontsize=title_font_size)

    # Add a grid
    plt.grid(True)

    # Increase tick font size
    plt.xticks(fontsize=x_tick_size)
    plt.yticks(fontsize=x_tick_size)

    # Increase the size of the ticks 
    plt.tick_params(axis='both', width=tick_width, length=tick_length)
    # Increase the size of the minor ticks
    plt.tick_params(axis='both', which='minor', width=tick_width/2, length=tick_length/2)

    # Increase the size of the border
    plt.gca().spines['top'].set_linewidth(border_width_upper_right)
    plt.gca().spines['right'].set_linewidth(border_width_upper_right)
    plt.gca().spines['bottom'].set_linewidth(border_width_x_y)
    plt.gca().spines['left'].set_linewidth(border_width_x_y)

    # Make the ticks point inwards
    plt.gca().tick_params(direction='in')
    # Make the minor ticks point inwards
    plt.gca().tick_params(which='minor', direction='in')

    # Save the plot as a PNG
    plt.tight_layout()
    plt.savefig('target/%s.png' % y, dpi=600)

#%% Plot the performance of egg
plot_loglog('tree_depth', 'egg_time_secs', color1)
plot_loglog('tree_depth', 'generate_tree_secs', color2)