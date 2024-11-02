import numpy as np
import matplotlib.pyplot as plt
import matplotlib.patches as patches
import matplotlib.cm as cm
import matplotlib.colors as mcolors

def recaman_sequence(n):
    sequence = [0]
    seen = set(sequence)
    for i in range(1, n):
        next_value = sequence[-1] - i
        if next_value > 0 and next_value not in seen:
            sequence.append(next_value)
        else:
            sequence.append(sequence[-1] + i)
        seen.add(sequence[-1])
    return sequence

# Circular diagram visualization showing the sequence path with colored arcs

def sequence_path_circular_visualization(sequence, base_radius=50, line_weight=0.5, alpha=0.5, save_path=None):
    # Prepare the plot
    fig, ax = plt.subplots(figsize=(12, 12))
    ax.set_aspect('equal')
    ax.axis('off')

    # Define group ranges and corresponding radii
    group_ranges = [(0, 60), (11, 360), (361, 360*5), (360*5+1, 10000)]
    radii = [base_radius * (i + 1) for i in range(len(group_ranges))]

    # Calculate positions for each index in the sequence
    positions = {}
    for idx, (start, end) in enumerate(group_ranges):
        radius = radii[idx]
        group_sequence = [value for value in sequence if start <= value <= end]
        n = len(group_sequence)
        if n == 0:
            continue
        # Start at the top and move clockwise
        angles = np.linspace(np.pi / 2, -3 * np.pi / 2, n, endpoint=False)

        # Calculate x and y coordinates for each point along the circle
        x_coords = radius * np.cos(angles)
        y_coords = radius * np.sin(angles)

        # Store the position of each value
        for i, value in enumerate(group_sequence):
            positions[value] = (x_coords[i], y_coords[i])

    # Define a color map for gradients
    norm = mcolors.Normalize(vmin=0, vmax=len(sequence))
    cmap = cm.get_cmap('viridis')

    # Draw the path of the Recamán sequence with colored arcs
    for i in range(1, len(sequence)):
        start_value = sequence[i - 1]
        end_value = sequence[i]

        if start_value in positions and end_value in positions:
            start_x, start_y = positions[start_value]
            end_x, end_y = positions[end_value]

            # Calculate the midpoint and control point for the arc
            center_x = (start_x + end_x) / 2
            center_y = (start_y + end_y) / 2
            radius = np.sqrt((end_x - start_x) ** 2 + (end_y - start_y) ** 2) / 2
            angle = np.arctan2(end_y - start_y, end_x - start_x)

            # Determine color based on index
            color = cmap(norm(i))

            # Add an arc between the points with gradient color
            arc = patches.Arc((center_x, center_y), width=2*radius, height=2*radius,
                              theta1=np.degrees(angle) - 90, theta2=np.degrees(angle) + 90,
                              color=color, lw=line_weight, alpha=alpha)
            ax.add_patch(arc)

    # Set limits to accommodate all circles
    max_radius = max(radii)
    ax.set_xlim(-max_radius - 20, max_radius + 20)
    ax.set_ylim(-max_radius - 20, max_radius + 20)

    plt.tight_layout()
    if save_path:
        plt.savefig(save_path, transparent=True, format='png')
    plt.show()

def sequence_path_circular_visualization(sequence, base_radius=50, line_weight=0.5, alpha=0.5, save_path=None):
    # Prepare the plot
    fig, ax = plt.subplots(figsize=(12, 12))
    ax.set_aspect('equal')
    ax.axis('off')

    # Define group ranges and corresponding radii
    group_ranges = [(0, 60), (11, 360), (361, 360*5), (360*5+1, 10000)]
    radii = [base_radius * (i + 1) for i in range(len(group_ranges))]

    # Calculate positions for each index in the sequence
    positions = {}
    for idx, (start, end) in enumerate(group_ranges):
        radius = radii[idx]
        group_sequence = [value for value in sequence if start <= value <= end]
        n = len(group_sequence)
        if n == 0:
            continue
        # Start at the top and move clockwise
        angles = np.linspace(np.pi / 2, -3 * np.pi / 2, n, endpoint=False)

        # Calculate x and y coordinates for each point along the circle
        x_coords = radius * np.cos(angles)
        y_coords = radius * np.sin(angles)

        # Store the position of each value
        for i, value in enumerate(group_sequence):
            positions[value] = (x_coords[i], y_coords[i])

    # Define a color map for gradients
    norm = mcolors.Normalize(vmin=0, vmax=len(sequence))
    cmap = cm.get_cmap('viridis')

    # Draw the path of the Recamán sequence with colored arcs
    for i in range(1, len(sequence)):
        start_value = sequence[i - 1]
        end_value = sequence[i]

        if start_value in positions and end_value in positions:
            start_x, start_y = positions[start_value]
            end_x, end_y = positions[end_value]

            # Determine the sweep direction
            sweep_flag = 0 if end_value > start_value else 1

            # Calculate radius for the arc
            radius = np.sqrt((end_x - start_x) ** 2 + (end_y - start_y) ** 2) / 2
            center_x = (start_x + end_x) / 2
            center_y = (start_y + end_y) / 2

            # Determine color based on index
            color = cmap(norm(i))

            # Add an arc between the points with gradient color
            arc = patches.Arc((center_x, center_y), width=2*radius, height=2*radius,
                              theta1=0, theta2=180 if sweep_flag == 0 else -180,
                              color=color, lw=line_weight, alpha=alpha)
            ax.add_patch(arc)

    # Set limits to accommodate all circles
    max_radius = max(radii)
    ax.set_xlim(-max_radius - 20, max_radius + 20)
    ax.set_ylim(-max_radius - 20, max_radius + 20)

    plt.tight_layout()
    if save_path:
        plt.savefig(save_path, transparent=True, format='png')
    plt.show()

def sequence_path_circular_visualization(sequence, base_radius=50, line_weight=0.5, alpha=0.5, save_path=None):
    # Prepare the plot
    fig, ax = plt.subplots(figsize=(12, 12))
    ax.set_aspect('equal')
    ax.axis('off')

    # Define group ranges and corresponding radii
    group_ranges = [(0, 60), (61, 360), (361, 360*5), (360*5+1, 10000)]
    radii = [base_radius * (i + 1) for i in range(len(group_ranges))]

    # Calculate positions for each index in the sequence
    positions = {}
    for idx, (start, end) in enumerate(group_ranges):
        radius = radii[idx]
        group_sequence = [value for value in sequence if start <= value <= end]
        n = len(group_sequence)
        if n == 0:
            continue
        # Start at the top and move clockwise
        angles = np.linspace(np.pi / 2, -3 * np.pi / 2, n, endpoint=False)

        # Calculate x and y coordinates for each point along the circle
        x_coords = radius * np.cos(angles)
        y_coords = radius * np.sin(angles)

        # Store the position of each value
        for i, value in enumerate(group_sequence):
            positions[value] = (x_coords[i], y_coords[i])

    # Define a color map for gradients
    norm = mcolors.Normalize(vmin=0, vmax=len(sequence))
    cmap = cm.get_cmap('viridis')

    # Draw the path of the Recamán sequence with continuous arcs
    for i in range(1, len(sequence)):
        start_value = sequence[i - 1]
        end_value = sequence[i]

        if start_value in positions and end_value in positions:
            start_x, start_y = positions[start_value]
            end_x, end_y = positions[end_value]

            # Calculate control points to create a smooth arc from start to end
            control_x = (start_x + end_x) / 2
            control_y = (start_y + end_y) / 2 + 0.3 * (end_x - start_x)

            # Determine color based on index
            color = cmap(norm(i))

            # Draw Bezier curve as an arc between points
            path_data = [
                (patches.Path.MOVETO, (start_x, start_y)),
                (patches.Path.CURVE3, (control_x, control_y)),
                (patches.Path.CURVE3, (end_x, end_y)),
            ]
            codes, verts = zip(*path_data)
            path = patches.Path(verts, codes)
            patch = patches.PathPatch(path, facecolor='none', lw=line_weight, edgecolor=color, alpha=alpha)
            ax.add_patch(patch)

    # Set limits to accommodate all circles
    max_radius = max(radii)
    ax.set_xlim(-max_radius - 20, max_radius + 20)
    ax.set_ylim(-max_radius - 20, max_radius + 20)

    plt.tight_layout()
    if save_path:
        plt.savefig(save_path, transparent=True, format='png')
    plt.show()

def sequence_path_circular_visualization(sequence, base_radius=50, line_weight=0.5, alpha=0.3, group_growth=10, save_path=None, transparent=True):
    # Prepare the plot
    fig, ax = plt.subplots(figsize=(12, 12))
    ax.set_aspect('equal')
    ax.axis('off')

    # Define group ranges and corresponding radii
    group_ranges = [(0, 60)]
    current_max = 60
    while current_max < max(sequence):
        next_min = current_max + 1
        next_max = current_max * group_growth  # Each new group is an order of magnitude larger
        group_ranges.append((next_min, next_max))
        current_max = next_max
    radii = [base_radius * (i + 1) for i in range(len(group_ranges))]

    # Calculate positions for each index in the sequence
    positions = {}
    for idx, (start, end) in enumerate(group_ranges):
        radius = radii[idx]
        group_sequence = [value for value in sequence if start <= value <= end]
        n = len(group_sequence)
        if n == 0:
            continue
        # Start at the top and move clockwise
        angles = np.linspace(np.pi / 2, -3 * np.pi / 2, n, endpoint=False)

        # Calculate x and y coordinates for each point along the circle
        x_coords = radius * np.cos(angles)
        y_coords = radius * np.sin(angles)

        # Store the position of each value
        for i, value in enumerate(group_sequence):
            positions[value] = (x_coords[i], y_coords[i])

    # Define a color map for gradients
    norm = mcolors.Normalize(vmin=0, vmax=len(sequence))
    cmap = cm.get_cmap('viridis')

    # Draw the path of the Recamán sequence with continuous arcs
    for i in range(1, len(sequence)):
        start_value = sequence[i - 1]
        end_value = sequence[i]

        if start_value in positions and end_value in positions:
            start_x, start_y = positions[start_value]
            end_x, end_y = positions[end_value]

            # Calculate control points to create a smooth arc from start to end
            control_x = (start_x + end_x) / 2
            control_y = (start_y + end_y) / 2 + 0.3 * (end_x - start_x)

            # Determine color based on index
            color = cmap(norm(i))

            # Draw Bezier curve as an arc between points
            path_data = [
                (patches.Path.MOVETO, (start_x, start_y)),
                (patches.Path.CURVE3, (control_x, control_y)),
                (patches.Path.CURVE3, (end_x, end_y)),
            ]
            codes, verts = zip(*path_data)
            path = patches.Path(verts, codes)
            patch = patches.PathPatch(path, facecolor='none', lw=line_weight, edgecolor=color, alpha=alpha)
            ax.add_patch(patch)

    # Set limits to accommodate all circles
    max_radius = max(radii)
    ax.set_xlim(-max_radius - 20, max_radius + 20)
    ax.set_ylim(-max_radius - 20, max_radius + 20)

    plt.tight_layout()
    if save_path:
        plt.savefig(save_path, transparent=transparent, format='png')
    plt.show()




# Test to verify the sequence generator matches expected values
def test_recaman_sequence():
    expected_values = [0, 1, 3, 6, 2, 7, 13, 20, 12, 21, 11, 22, 10, 23, 9, 24, 8, 25, 43, 62, 42, 63, 41, 18, 42, 17, 43, 16, 44, 15, 45, 14, 46, 79, 113, 78, 114, 77, 39, 78, 38, 79, 37, 80, 36, 81, 35, 82, 34, 83, 33, 84, 32, 85, 31, 86, 30, 87, 29, 88, 28, 89, 27, 90, 26, 91, 157, 224, 156, 225, 155]
    generated_sequence = recaman_sequence(len(expected_values))
    assert generated_sequence == expected_values, f"Test failed: {generated_sequence} != {expected_values}"
    print("Test passed: The Recamán sequence matches the expected values.")

# Run the test



if __name__ == "__main__":
    test_recaman_sequence()

    # Generate the Recamán sequence with 10,000 elements
    elements = 6000
    alpha = 0.2
    group_growth = 6
    transparent = False
    recaman_seq = recaman_sequence(elements)

    # Plot the circular visualization showing the sequence path with colored arcs and save to PNG
    #sequence_path_circular_visualization(recaman_seq, save_path='/mnt/data/recaman_sequence_path_visualization_10000.png')


    # Plot the circular visualization showing the sequence path with colored arcs and save to PNG with transparency
    sequence_path_circular_visualization(recaman_seq, alpha=alpha, transparent=transparent, group_growth=group_growth, save_path=f'recaman-sequence-colour-[elements={elements}][alpha={alpha}][growth={group_growth}].png', )
