"use strict";

// Handle menu toggle
document.addEventListener('DOMContentLoaded', () => {
    // Get the menu button and sidebar elements
    const menuButton = document.querySelector('.menu-bar-button');
    const sidebar = document.querySelector('.sidebar');
    
    // Add click handler to toggle menu visibility
    if (menuButton && sidebar) {
        menuButton.addEventListener('click', (e) => {
            // Only toggle if Ctrl key is pressed
            if (e.ctrlKey) {
                e.preventDefault(); // Prevent any default ctrl+click behavior
                sidebar.classList.toggle('hidden');
                
                // Store the state in localStorage to persist across page loads
                const isHidden = sidebar.classList.contains('hidden');
                localStorage.setItem('sidebar-hidden', isHidden);
            }
        });
        
        // Restore sidebar state from localStorage on page load
        const wasHidden = localStorage.getItem('sidebar-hidden') === 'true';
        if (wasHidden) {
            sidebar.classList.add('hidden');
        }
    }
    
    // Add keyboard shortcut (Alt+M) to toggle menu
    document.addEventListener('keydown', (e) => {
        if (e.altKey && e.key === 'm' && menuButton && sidebar) {
            e.preventDefault();
            menuButton.click();
        }
    });
});