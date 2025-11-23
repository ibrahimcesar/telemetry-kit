/**
 * Example with automatic sync enabled
 *
 * This demonstrates:
 * - Auto-sync configuration
 * - Sync credentials
 * - Background synchronization
 * - Manual sync trigger
 */

async function main() {
  console.log('🔭 telemetry-kit with Sync Example\n');

  // Create telemetry with sync enabled
  console.log('Creating telemetry instance with sync...');
  const telemetry = new TelemetryKit({
    serviceName: 'example-sync-app',
    autoSync: true,
    syncInterval: 60, // Sync every 60 seconds
    syncOnShutdown: true,
    syncConfig: {
      organizationId: '550e8400-e29b-41d4-a716-446655440000',
      applicationId: '6ba7b810-9dad-11d1-80b4-00c04fd430c8',
      token: 'your-token-here',
      secret: 'your-secret-here',
      endpoint: 'http://localhost:3000', // Optional custom endpoint
    },
  });

  console.log('✅ Auto-sync enabled (every 60s)\n');

  // Track some events
  console.log('Tracking events...');
  for (let i = 0; i < 5; i++) {
    await telemetry.trackCommand('api-call', {
      success: Math.random() > 0.2, // 80% success rate
      durationMs: Math.floor(Math.random() * 1000) + 100,
    });

    console.log(`  Event ${i + 1}/5 tracked`);
  }

  // Manual sync
  console.log('\nTriggering manual sync...');
  try {
    await telemetry.sync();
    console.log('✅ Sync successful');
  } catch (error) {
    console.error('⚠️ Sync failed (expected if server not running):', error.message);
  }

  // Statistics
  const stats = await telemetry.stats();
  console.log('\nStatistics:');
  console.log(`  Total: ${stats.total}`);
  console.log(`  Synced: ${stats.synced}`);
  console.log(`  Unsynced: ${stats.unsynced}`);

  // Shutdown (will trigger final sync)
  console.log('\nShutting down (final sync)...');
  await telemetry.shutdown();

  console.log('✅ Done!\n');
}

if (require.main === module) {
  main().catch((error) => {
    console.error('❌ Error:', error.message);
    process.exit(1);
  });
}

module.exports = { main };
