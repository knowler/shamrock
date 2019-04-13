<?php
/**
 * Plugin Name:     {{ plugin_name }}
 * Description:     {{ description }}
 * Version:         {{ version }}
 * Author:          {{ author }}
 * Author URI:      {{ author_uri }}
 * License:         {{ license }}
 * Text Domain:     {{ text_domain }}
 *
 * Requires PHP 7+
 */

/** Create the plugin */
$plugin = new class {

    /**
     * Plugin runtime
     */
    public function run()
    {
        /** Run when the plugin activates */
        register_activation_hook(__FILE__, function () {
            // --
        });

        /** Run when the plugin deactivates */
        register_deactivation_hook(__FILE__, function () {
            // --
        });
    }
};

/** Run the plugin */
$plugin->run();
