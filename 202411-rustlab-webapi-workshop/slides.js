// "use strict";

// const slideConfig = {
//     slideSelector: 'section.slide',
//     scrollDuration: 500,
//     keys: {
//         next: ['ArrowDown'],  // Down arrow and spacebar
//         prev: ['ArrowUp'],         // Up arrow
//         all: ['ArrowDown', 'ArrowUp']  // All navigation keys
//     }
// };

// (function slideNavigation() {

//     let slides = [];
//     let currentSlideIndex = 0;
//     let isScrolling = false;

//     function initSlideNavigation() {
//         slides = Array.from(document.querySelectorAll(slideConfig.slideSelector));
        
//         if (slides.length === 0) return;

//         currentSlideIndex = findCurrentSlideIndex();

//         // Prevent default scrolling behavior for navigation keys
//         document.addEventListener('keydown', (event) => {
//             if (slideConfig.keys.all.includes(event.key) && 
//                 !event.altKey && !event.ctrlKey && !event.metaKey && !event.shiftKey) {
//                 event.preventDefault();
//             }
//         }, { passive: false });

//         // Handle navigation
//         document.addEventListener('keydown', handleKeyPress);

//         // Handle direct scrolling
//         document.addEventListener('scroll', debounce(() => {
//             if (!isScrolling) {
//                 currentSlideIndex = findCurrentSlideIndex();
//             }
//         }, 150));
//     }

//     function debounce(func, wait) {
//         let timeout;
//         return function executedFunction(...args) {
//             const later = () => {
//                 clearTimeout(timeout);
//                 func(...args);
//             };
//             clearTimeout(timeout);
//             timeout = setTimeout(later, wait);
//         };
//     }

//     function findCurrentSlideIndex() {
//         const viewportMiddle = window.scrollY + (window.innerHeight / 2);
        
//         for (let i = 0; i < slides.length; i++) {
//             const rect = slides[i].getBoundingClientRect();
//             const absoluteTop = rect.top + window.scrollY;
//             const absoluteBottom = absoluteTop + rect.height;
            
//             if (viewportMiddle >= absoluteTop && viewportMiddle < absoluteBottom) {
//                 return i;
//             }
//         }
//         return 0;
//     }

//     function handleKeyPress(event) {
//         // Don't handle if modifier keys are pressed
//         if (event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) return;
        
//         // Don't handle if we're in a search box or similar
//         if (event.target.tagName === 'INPUT' || event.target.tagName === 'TEXTAREA') return;
        
//         if (isScrolling) return;

//         if (slideConfig.keys.next.includes(event.key)) {
//             navigateToSlide(currentSlideIndex + 1);
//         } else if (slideConfig.keys.prev.includes(event.key)) {
//             navigateToSlide(currentSlideIndex - 1);
//         }
//     }

//     function navigateToSlide(targetIndex) {
//         if (targetIndex < 0 || targetIndex >= slides.length) return;
        
//         isScrolling = true;
//         const targetSlide = slides[targetIndex];
//         const targetPosition = targetSlide.getBoundingClientRect().top + window.scrollY;
        
//         window.scrollTo({
//             top: targetPosition,
//             behavior: 'smooth'
//         });
        
//         currentSlideIndex = targetIndex;

//         setTimeout(() => {
//             isScrolling = false;
//         }, slideConfig.scrollDuration);
//     }

//     // Initialize when document is ready
//     if (document.readyState === 'loading') {
//         document.addEventListener('DOMContentLoaded', initSlideNavigation);
//     } else {
//         initSlideNavigation();
//     }
// })();


// First, add this CSS to your stylesheet
const fragmentStyles = `
.fragment {
  opacity: 0;
  transition: opacity 0.3s ease;
}

.fragment.visible {
  opacity: 1;
}
`;

// Add the styles to the document
const styleSheet = document.createElement("style");
styleSheet.textContent = fragmentStyles;
document.head.appendChild(styleSheet);

"use strict";
const slideConfig = {
  slideSelector: 'section.slide',
  fragmentSelector: '.fragment',
  scrollDuration: 500,
  keys: {
    next: ['ArrowDown', 'ArrowRight'],
    prev: ['ArrowUp', 'ArrowLeft'],
    all: ['ArrowDown', 'ArrowUp', 'ArrowRight', 'ArrowLeft']
  }
};

(function slideNavigation() {
  let slides = [];
  let currentSlideIndex = 0;
  let currentFragmentIndex = -1;
  let isScrolling = false;

  function initSlideNavigation() {
    slides = Array.from(document.querySelectorAll(slideConfig.slideSelector));
    if (slides.length === 0) return;
    
    currentSlideIndex = findCurrentSlideIndex();
    
    // Initialize fragments for current slide
    resetFragments();
    
    document.addEventListener('keydown', (event) => {
      if (slideConfig.keys.all.includes(event.key) &&
          !event.altKey && !event.ctrlKey && !event.metaKey && !event.shiftKey) {
        event.preventDefault();
      }
    }, { passive: false });
    
    document.addEventListener('keydown', handleKeyPress);
    document.addEventListener('scroll', debounce(() => {
      if (!isScrolling) {
        const newSlideIndex = findCurrentSlideIndex();
        if (newSlideIndex !== currentSlideIndex) {
          currentSlideIndex = newSlideIndex;
          resetFragments();
        }
      }
    }, 150));
  }

  function resetFragments() {
    // Hide all fragments in current slide
    const currentSlide = slides[currentSlideIndex];
    const fragments = currentSlide.querySelectorAll(slideConfig.fragmentSelector);
    fragments.forEach(fragment => {
      fragment.classList.remove('visible');
    });
    currentFragmentIndex = -1;
  }

  function showNextFragment() {
    const currentSlide = slides[currentSlideIndex];
    const fragments = Array.from(currentSlide.querySelectorAll(slideConfig.fragmentSelector));
    
    if (fragments.length === 0) return false;
    if (currentFragmentIndex >= fragments.length - 1) return false;
    
    currentFragmentIndex++;
    fragments[currentFragmentIndex].classList.add('visible');
    return true;
  }

  function hideLastFragment() {
    const currentSlide = slides[currentSlideIndex];
    const fragments = Array.from(currentSlide.querySelectorAll(slideConfig.fragmentSelector));
    
    if (fragments.length === 0) return false;
    if (currentFragmentIndex < 0) return false;
    
    fragments[currentFragmentIndex].classList.remove('visible');
    currentFragmentIndex--;
    return true;
  }

  function handleKeyPress(event) {
    if (event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) return;
    if (event.target.tagName === 'INPUT' || event.target.tagName === 'TEXTAREA') return;
    if (isScrolling) return;

    if (slideConfig.keys.next.includes(event.key)) {
      // Try to show next fragment first
      if (!showNextFragment()) {
        // If no more fragments, go to next slide
        navigateToSlide(currentSlideIndex + 1);
      }
    } else if (slideConfig.keys.prev.includes(event.key)) {
      // Try to hide last fragment first
      if (!hideLastFragment()) {
        // If no more fragments to hide, go to previous slide
        navigateToSlide(currentSlideIndex - 1);
      }
    }
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
    resetFragments();
    
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