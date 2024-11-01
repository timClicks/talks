"use strict";

const slideConfig = {
    slideSelector: 'section.slide',
    scrollDuration: 500,
    keys: {
        next: ['ArrowDown'],  // Down arrow and spacebar
        prev: ['ArrowUp'],         // Up arrow
        all: ['ArrowDown', 'ArrowUp']  // All navigation keys
    }
};

(function slideNavigation() {

    let slides = [];
    let currentSlideIndex = 0;
    let isScrolling = false;

    function initSlideNavigation() {
        slides = Array.from(document.querySelectorAll(slideConfig.slideSelector));
        
        if (slides.length === 0) return;

        currentSlideIndex = findCurrentSlideIndex();

        // Prevent default scrolling behavior for navigation keys
        document.addEventListener('keydown', (event) => {
            if (slideConfig.keys.all.includes(event.key) && 
                !event.altKey && !event.ctrlKey && !event.metaKey && !event.shiftKey) {
                event.preventDefault();
            }
        }, { passive: false });

        // Handle navigation
        document.addEventListener('keydown', handleKeyPress);

        // Handle direct scrolling
        document.addEventListener('scroll', debounce(() => {
            if (!isScrolling) {
                currentSlideIndex = findCurrentSlideIndex();
            }
        }, 150));
    }

    function debounce(func, wait) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    }

    function findCurrentSlideIndex() {
        const viewportMiddle = window.scrollY + (window.innerHeight / 2);
        
        for (let i = 0; i < slides.length; i++) {
            const rect = slides[i].getBoundingClientRect();
            const absoluteTop = rect.top + window.scrollY;
            const absoluteBottom = absoluteTop + rect.height;
            
            if (viewportMiddle >= absoluteTop && viewportMiddle < absoluteBottom) {
                return i;
            }
        }
        return 0;
    }

    function handleKeyPress(event) {
        // Don't handle if modifier keys are pressed
        if (event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) return;
        
        // Don't handle if we're in a search box or similar
        if (event.target.tagName === 'INPUT' || event.target.tagName === 'TEXTAREA') return;
        
        if (isScrolling) return;

        if (slideConfig.keys.next.includes(event.key)) {
            navigateToSlide(currentSlideIndex + 1);
        } else if (slideConfig.keys.prev.includes(event.key)) {
            navigateToSlide(currentSlideIndex - 1);
        }
    }

    function navigateToSlide(targetIndex) {
        if (targetIndex < 0 || targetIndex >= slides.length) return;
        
        isScrolling = true;
        const targetSlide = slides[targetIndex];
        const targetPosition = targetSlide.getBoundingClientRect().top + window.scrollY;
        
        window.scrollTo({
            top: targetPosition,
            behavior: 'smooth'
        });
        
        currentSlideIndex = targetIndex;

        setTimeout(() => {
            isScrolling = false;
        }, slideConfig.scrollDuration);
    }

    // Initialize when document is ready
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initSlideNavigation);
    } else {
        initSlideNavigation();
    }
})();
