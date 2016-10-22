package me.telegram.borken_bot.commands;

import lib.TestUtils;
import org.junit.Before;
import org.junit.Ignore;
import org.junit.Test;
import org.mockito.ArgumentCaptor;
import org.telegram.telegrambots.api.methods.send.SendMessage;
import org.telegram.telegrambots.bots.AbsSender;
import org.telegram.telegrambots.exceptions.TelegramApiException;

import static lib.TestUtils.executeSimple;
import static org.junit.Assert.*;
import static org.mockito.Mockito.*;

public class MagicBallTest {
    private MagicBall ball;

    @Before
    public void setUp() {
        ball = new MagicBall();
    }

    @Test
    @Ignore
    public void testGetAnswers() throws TelegramApiException {
        ball = spy(ball);
        doReturn(5).when(ball).getRandom();

        ArgumentCaptor<SendMessage> messageCaptor = ArgumentCaptor.forClass(SendMessage.class);

        AbsSender sender = mock(AbsSender.class);

        executeSimple(sender, ball, new String[]{"8"});
        verify(sender).sendMessage(messageCaptor.capture());

        SendMessage message = messageCaptor.getValue();

        assertEquals("Мне кажется - «да»", message.getText());
    }

    @Test
    public void testIsValidAction() {
        assertTrue(ball.isValidAction("8", null));
        assertTrue(ball.isValidAction("ball", null));
        assertTrue(ball.isValidAction("8ball", null));
        assertFalse(ball.isValidAction("_8ball", null));
    }

    @Test
    public void testGetShortNotations() {
        String[] notations = ball.getShortNotations();

        assertEquals(2, notations.length);
        assertEquals("8", notations[0]);
        assertEquals("ball", notations[1]);
    }

    @Test
    public void testConflictCommands() {
        assertFalse(TestUtils.isConflict(ball));
    }
}